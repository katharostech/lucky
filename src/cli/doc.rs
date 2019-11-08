//! Handles printing bighelp pages

use crossterm::{
    cursor::{Hide, Show},
    input::{input, InputEvent::*, KeyEvent::*},
    queue,
    screen::{EnterAlternateScreen, LeaveAlternateScreen, RawScreen},
    style::{style, Attribute::*, Color, Color::*},
};
use std::io::{stdout, Read, Write, Seek, SeekFrom};
use std::fs::OpenOptions;
use std::collections::HashMap;
use anyhow::Context;
use termimad::*;

pub(crate) fn get_markdown_skin() -> MadSkin {
    let mut skin = MadSkin::default();
        skin.set_headers_fg(DarkYellow);
        skin.bold.set_fg(Magenta);
        skin.italic.add_attr(Underlined);
        // Clear code block formatting for now because we are inlining the help and it gets its
        // styling messed up. See: https://github.com/Canop/termimad/issues/8.
        // TODO: Fix that so that we can use inline code styling again.
        skin.inline_code.set_bg(Color::Reset);
        skin.code_block.set_bg(Color::Reset);
        skin.code_block.align = Alignment::Left;
    
    skin
}

lazy_static::lazy_static!{
    static ref USAGE_TEMPLATE: String = {
        let usage_header = style("USAGE:").with(DarkYellow);
        format!("{} {{usage}}\n\n{{all-args}}", usage_header)
    };
}

/// Render the document
/// @param command      The clap App that you are printing help for. Used to print help info in doc
/// @param doc_name     Used to save the position that the user has scrolled to for that doc
/// @param document     The markdown document to render
pub(crate) fn run(mut command: clap::App, doc_name: &str, document: &str) -> anyhow::Result<()> {
    // Hide help, doc, and version flags in command help
    command = command
        .mut_arg("help", |arg| arg.hidden_long_help(true))
        .mut_arg("doc", |arg| arg.hidden_long_help(true))
        .mut_arg("version", |arg| arg.hidden_long_help(true));
    
    // Insert help message into document
    let mut help_message = String::new();
    unsafe { // This is OK because we know that `write_long_help` will produce valid UTF-8
        
        command.template = Some(&USAGE_TEMPLATE);
        command.write_long_help(help_message.as_mut_vec())?;
    }
    let document = document.replace("{{help_message}}", &help_message);

    // Create a doc skin
    let skin = get_markdown_skin();

    // If this is a tty
    if atty::is(atty::Stream::Stdout) {
        // Load the last position the user was scrolled to on this doc
        let mut scrolled_positions: HashMap<String, i32> = HashMap::new();
        let mut config_file: Option<std::fs::File> = None;
        if let Some(config_dir) = dirs::config_dir() {
            // Open config file
            let mut config_path = config_dir.clone();
            config_path.push("lucky_doc_positions.yml");
            let mut file = OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(&config_path)
                .context(format!("Couldn't open config file: {:?}", &config_path))?;
            let mut config_content = String::new();
            file.read_to_string(&mut config_content)?;

            // If the config file contains readable YAML
            if let Ok(positions) = serde_yaml::from_str(&config_content) {
                scrolled_positions = positions;

            // If we can't parse the config, we just leave it initialized as an empty HashMap
            }

            // Set config file for use later
            config_file = Some(file);
        }

        // Switch to the Pager Screen
        let mut w = stdout();
        queue!(w, EnterAlternateScreen)?;
        let _raw = RawScreen::into_raw_mode()?;
        queue!(w, Hide)?;

        // Create a scrollable area for the markdown renderer
        let mut area = Area::full_screen();
        area.pad(1, 1);
        let mut view = MadView::from(document.to_owned(), area, skin);

        // Scroll view to the last viewed position
        if let Some(&pos) = scrolled_positions.get(doc_name) {
            view.write_on(&mut w)?;
            view.try_scroll_lines(pos);
        }

        // Skipping the help message for now unless we decide it is a good idea.
        // /// Print the pager help message
        // fn print_help(w: &mut dyn Write) -> anyhow::Result<()> {
        //     // Add the help message to the bottom of the viewer
        //     if let Some(size) = termsize::get() {
        //         queue!(w, MoveTo(0, size.rows))?;
        //     } else {
        //         queue!(w, MoveTo(0, 0))?;
        //     }
        //     queue!(w, PrintStyledContent(
        //         style("Type Enter to exit")
        //         .with(Black)
        //         .on(Grey)
        //     ))?;
        //     Ok(())
        // }

        // print_help(&mut w)?;

        // Listen for events and redraw screen
        let mut events = input().read_sync();
        loop {
            view.write_on(&mut w)?;
            // print_help(&mut w)?;

            if let Some(Keyboard(key)) = events.next() {
                match key {
                    Home | Char('g') => view.scroll = 0,
                    End | Char('G') => view.try_scroll_pages(1000), // There might be a better way to get to the end
                    Up | Char('k') => view.try_scroll_lines(-1),
                    Down | Char('j') => view.try_scroll_lines(1),
                    PageUp => view.try_scroll_pages(-1),
                    PageDown => view.try_scroll_pages(1),
                    Esc | Enter | Char('q') => break,
                    _ => (),
                }
                w.flush()?;
            }
        }

        // Set our new latest scroll position for this document
        scrolled_positions.insert(doc_name.to_owned(), view.scroll);

        // Save scrolled positions to config file
        if let Some(mut file) = config_file {
            // Clear the file and go to the beginning
            file.set_len(0)?;
            file.seek(SeekFrom::Start(0))?;

            // Write out the new scrolled positions
            serde_yaml::to_writer(&file, &scrolled_positions)?;
            file.sync_all()?;
        }

        // Clean up revert screen
        queue!(w, Show)?;
        queue!(w, LeaveAlternateScreen)?;
        w.flush()?;

    // If this isn't a tty
    } else {
        // Print page
        // NOTE: This will still print out the colors so that you can pipe
        // the output to `less -R` or `cat` and still get the color.
        skin.write_text(&document)?;
    }

    // Exit process
    std::process::exit(0);
}

/// Return the `doc` subcommand
pub(crate) fn get_arg<'a>() -> clap::Arg<'a> {
    clap::Arg::with_name("doc")
        .help("Show the detailed command documentation ( similar to a man page )")
        .long("doc")
        .short('H')
        .long_help(include_str!("doc/long_help.txt"))
}
