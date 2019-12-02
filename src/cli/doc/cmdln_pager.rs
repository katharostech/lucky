use crate::cli::CliError;
use anyhow::Context;
use crossterm::{
    cursor::{Hide, Show},
    input::{input, InputEvent::*, KeyEvent::*},
    queue,
    screen::{EnterAlternateScreen, LeaveAlternateScreen, RawScreen},
    style::{style, Attribute::*, Color::*},
    terminal::{Clear, ClearType::All},
};
use minimad::TextTemplate;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::{stdout, Read, Seek, SeekFrom, Write};
use termimad::*;

use crate::cli::CliDoc;

lazy_static::lazy_static! {
    /// Creates a colored `USAGE: ` + args template for use in the doc pages
    static ref USAGE_TEMPLATE: String = {
        let usage_header = style("USAGE:").with(DarkYellow);
        format!("{} {{usage}}\n\n{{all-args}}", usage_header)
    };
}

/// Get the markdown renderer skin
pub(crate) fn get_markdown_skin() -> MadSkin {
    let mut skin = MadSkin::default();
    skin.headers[0].set_fg(DarkYellow);
    skin.set_headers_fg(DarkYellow);
    skin.bold.set_fg(Magenta);
    skin.italic.add_attr(Underlined);

    skin
}

/// Show the commandline pager with documentation for the given command
pub(crate) fn show_doc_page<'a>(get_cli: impl Fn() -> clap::App<'a>, cli_doc: CliDoc) -> anyhow::Result<()> {
    // Hide the help, doc, and version flags in the command help message.
    // TODO: The command width is not recalculated on resize. We might want to do that, but it is
    // not a huge deal.
    let mut cli = get_cli()
        .mut_arg("help", |arg| arg.hidden_long_help(true))
        .mut_arg("doc", |arg| arg.hidden_long_help(true))
        .mut_arg("version", |arg| arg.hidden_long_help(true));

    // Set the help message template
    cli.template = Some(&USAGE_TEMPLATE);

    // Create Termimad template from document
    let doc_template = TextTemplate::from(cli_doc.content);
    let mut doc_expander = doc_template.expander();
    let mut help_message = vec![];
    cli.write_long_help(&mut help_message)
        .expect("Could not write to internal string buffer");
    let help_message =
        &String::from_utf8(help_message).expect("Could not parse command help as utf8");
    doc_expander.set_lines("help_message", help_message);

    // Expand document template
    let doc = doc_expander.expand();

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
            std::fs::create_dir_all(&config_path).context(format!(
                "Couldn't create config directory: {:?}",
                &config_path
            ))?;
            config_path.push("lucky_doc_positions.json");
            let mut file = OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(&config_path)
                .context(format!("Couldn't open config file: {:?}", &config_path))?;
            let mut config_content = String::new();
            file.read_to_string(&mut config_content)?;

            // If the config file contains readable JSON
            if let Ok(positions) = serde_json::from_str(&config_content) {
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

        // // Create a scrollable area for the markdown renderer
        let mut area = Area::full_screen();
        area.pad(1, 1);
        let fmt_text = FmtText::from_text(&skin, doc.clone(), Some((area.width - 1) as usize));
        let mut view = TextView::from(&area, &fmt_text);

        // // Keep track of changes to scroll and screensize
        let mut scroll = 0;
        let mut screen_size = (area.width, area.height);

        // Scroll to the last viewed position
        if let Some(&pos) = scrolled_positions.get(cli_doc.name) {
            view.write_on(&mut w)?;
            view.try_scroll_lines(pos);
            scroll = view.scroll;
        }

        // Listen for events and redraw screen
        let mut events = input().read_sync();
        loop {
            // Reload CLI in case the screen size changed and help message needs re-printing
            let mut cli = get_cli()
                .mut_arg("help", |arg| arg.hidden_long_help(true))
                .mut_arg("doc", |arg| arg.hidden_long_help(true))
                .mut_arg("version", |arg| arg.hidden_long_help(true));

            // Set the help message template
            cli.template = Some(&USAGE_TEMPLATE);

            // Create Termimad template from document
            let doc_template = TextTemplate::from(cli_doc.content);
            let mut doc_expander = doc_template.expander();
            let mut help_message = vec![];
            cli.write_long_help(&mut help_message)
                .expect("Could not write to internal string buffer");
            let help_message =
                &String::from_utf8(help_message).expect("Could not parse command help as utf8");
            doc_expander.set_lines("help_message", help_message);

            // Expand document template
            let doc = doc_expander.expand();

            // Prepare and write to scroll area
            let mut area = Area::full_screen();
            // Clear the screen if screen was resized
            if screen_size != (area.width, area.height) {
                queue!(w, Clear(All))?;
                screen_size = (area.width, area.height);
            }
            area.pad(1, 1);
            let fmt_text = FmtText::from_text(&skin, doc.clone(), Some((area.width - 1) as usize));
            let mut view = TextView::from(&area, &fmt_text);
            view.scroll = scroll;
            view.write_on(&mut w)?;
            w.flush()?;

            // Respond to keyboard events
            if let Some(Keyboard(key)) = events.next() {
                match key {
                    Home | Char('g') => {
                        view.scroll = 0;
                    }
                    // TODO: find be a better way to scroll to end of page
                    End | Char('G') => {
                        view.try_scroll_pages(90000);
                    }
                    Up | Char('k') => {
                        view.try_scroll_lines(-1);
                    }
                    Down | Char('j') => {
                        view.try_scroll_lines(1);
                    }
                    PageUp | Backspace => {
                        view.try_scroll_pages(-1);
                    }
                    PageDown | Char(' ') => {
                        view.try_scroll_pages(1);
                    }
                    Esc | Enter | Char('q') => break,
                    _ => (),
                }
                w.flush()?;
            }

            // Update our tracked scroll position
            scroll = view.scroll;
        }

        // Set our new latest scroll position for this document
        scrolled_positions.insert(cli_doc.name.to_owned(), view.scroll);

        // Save scrolled positions to config file
        if let Some(mut file) = config_file {
            // Clear the file and go to the beginning
            file.set_len(0)?;
            file.seek(SeekFrom::Start(0))?;

            // Write out the new scrolled positions
            serde_json::to_writer(&file, &scrolled_positions)?;
            file.sync_all()?;
        }

        // Clean up and revert screen
        queue!(w, Show)?;
        queue!(w, LeaveAlternateScreen)?;
        w.flush()?;

    // If this isn't a tty
    } else {
        // Print page
        // NOTE: This will still print out the colors so that you can pipe the output to `less -R`
        // or `cat` and still get the color. Open an issue of you think it should be different.
        skin.write_text(cli_doc.content)?;
    }

    // Exit process
    Err(CliError::Exit(0).into())
}
