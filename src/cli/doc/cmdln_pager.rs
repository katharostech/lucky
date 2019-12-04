use crate::cli::CliError;
use anyhow::Context;
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    input::{input, InputEvent::*, KeyEvent::*, SyncReader},
    queue,
    screen::{EnterAlternateScreen, LeaveAlternateScreen, RawScreen},
    style::{style, Attribute::*, Color::*, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{size, Clear, ClearType::All},
};
use lazy_static::lazy_static;
use minimad::TextTemplate;
use regex::Regex;
use termimad::*;

use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::{stdout, Read, Seek, SeekFrom, Write};

use crate::cli::{CliCommand, CliDoc};

lazy_static! {
    /// Creates a colored `USAGE: ` + args template for use in the doc pages
    static ref USAGE_TEMPLATE: String = {
        let usage_header = style("USAGE:").with(DarkYellow);
        format!("{} {{usage}}\n\n{{all-args}}", usage_header)
    };

    /// The markdown renderer skin
    static ref MD_SKIN: MadSkin = {
        let mut skin = MadSkin::default();
        skin.headers[0].set_fg(DarkYellow);
        skin.set_headers_fg(DarkYellow);
        skin.bold.set_fg(Magenta);
        skin.italic.add_attr(Underlined);

        skin
    };
}

/// Show the commandline pager with documentation for the given command
pub(crate) fn show_doc_page<'a>(command: &impl CliCommand<'a>) -> anyhow::Result<()> {
    // Hide the help, doc, and version flags in the command help message.
    let cli_doc = match command.get_doc() {
        Some(doc) => doc,
        None => anyhow::bail!("This command does not have a doc page yet"),
    };

    // Get stdout writer
    let mut w = stdout();

    // Print raw doc if page if this is not a tty. We might want to change this later.
    if !atty::is(atty::Stream::Stdout) {
        print_raw_doc(&mut w, cli_doc)?;
        return Ok(());
    }

    // Load the last position the user was scrolled to on this doc
    let mut scrolled_positions: HashMap<String, i32> = HashMap::new();
    let mut config_file: Option<std::fs::File> = None;
    if let Some(config_dir) = dirs::config_dir() {
        // Open config file
        let mut config_path = config_dir;
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
    queue!(w, EnterAlternateScreen)?;
    let _raw = RawScreen::into_raw_mode()?;
    queue!(w, Hide)?;

    // // Keep track of changes to scroll, screensize, and first view
    let mut scroll = 0;
    let mut screen_size = size()?;
    let mut first_view = true;

    // Listen for events and redraw screen
    let mut events = input().read_sync();
    loop {
        // Reload CLI in case the screen size changed and help message needs re-printing
        let mut cli = command
            .get_cli()
            .mut_arg("help", |arg| arg.hidden_long_help(true))
            .mut_arg("doc", |arg| arg.hidden_long_help(true))
            .mut_arg("version", |arg| arg.hidden_long_help(true));

        // Set the help message template
        cli.template = Some(&USAGE_TEMPLATE);

        // Create Termimad template from document
        let content = preprocess_markdown(cli_doc.content);
        let doc_template = TextTemplate::from(content.as_ref());
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

        // Pad text area and give room for help bar at bottom
        area.pad(1, 1);
        area.height -= 1;

        // Create text view
        let fmt_text = FmtText::from_text(&MD_SKIN, doc.clone(), Some((area.width - 1) as usize));
        let mut view = TextView::from(&area, &fmt_text);

        // Scroll to the last viewed position
        if first_view {
            if let Some(&pos) = scrolled_positions.get(cli_doc.name) {
                view.try_scroll_lines(pos);
                scroll = view.scroll;
            }
            first_view = false;
        } else {
            view.scroll = scroll;
        }

        // Write out the document view
        view.write_on(&mut w)?;

        // Write out help bar
        write_help_bar(&mut w, r#" Type "h" for help "#)?;

        // Flush output
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
                Char('h') | Char('?') => {
                    show_pager_help(&mut w, &mut events)?;
                    continue;
                }
                Esc | Enter | Char('q') => break,
                _ => (),
            }
        }

        // Update our tracked scroll position
        scroll = view.scroll;
    }

    // Set our new latest scroll position for this document
    scrolled_positions.insert(cli_doc.name.to_owned(), scroll);

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

    // Exit process
    Err(CliError::Exit(0).into())
}

/// Add a bar to the bottom of the terminal with the given message
fn write_help_bar(w: &mut impl Write, message: &str) -> anyhow::Result<()> {
    let screen_size = size()?;

    queue!(w, MoveTo(0, screen_size.1))?;
    queue!(w, SetBackgroundColor(Grey))?;
    queue!(w, SetForegroundColor(Black))?;
    write!(w, "{}", message)?;
    queue!(w, ResetColor)?;

    Ok(())
}

/// Prints out the raw documentation content without any formatting or colors
fn print_raw_doc(w: &mut impl Write, cli_doc: CliDoc) -> anyhow::Result<()> {
    write!(w, "{}", cli_doc.content)?;

    Ok(())
}

/// Show the pager controls help page
fn show_pager_help(mut w: &mut impl Write, events: &mut SyncReader) -> anyhow::Result<()> {
    // Clear screen
    queue!(w, Clear(All))?;

    let mut scroll = 0;
    loop {
        // Create screen area
        let mut area = Area::full_screen();
        area.pad(1, 1);
        area.height -= 1;

        // Create text view
        let fmt_text = FmtText::from_text(
            &MD_SKIN,
            include_str!("cmdln_pager/pager_help.md").into(),
            Some((area.width - 1) as usize),
        );
        let mut view = TextView::from(&area, &fmt_text);
        view.scroll = scroll;

        // Handle keyboard events
        write_help_bar(&mut w, r#" Type "Esc" to go back "#)?;
        view.write_on(&mut w)?;
        w.flush()?;

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

            scroll = view.scroll;
        }
    }

    // Clear screen
    queue!(w, Clear(All))?;

    Ok(())
}

lazy_static! {
    /// Matches a markdown link that starts with `http(s)://`
    static ref EXTERNAL_LINKS: Regex =
        Regex::new(r"(?m)\[(?P<link_text>.*?)\]\((?P<link_ref>http(s)?://.*?)\)")
            .expect("Coud not compile regex");

    /// Matches any markdown link
    static ref ALL_LINKS: Regex =
        Regex::new(r"(?m)\[(?P<link_text>.*?)\]\((?P<link_ref>.*?)\)")
            .expect("Coud not compile regex");
}

/// Pre-process the markdown doc
///
/// Reformats links look nicer in the terminal
fn preprocess_markdown(markdown: &str) -> String {
    // Reformat external links to make them prettier in terminal
    let first_pass = EXTERNAL_LINKS.replace_all(markdown, "$link_text ( *$link_ref* )");

    // Remove any links that don't start with `http(s)://` because they will not work in the
    // terminal.
    let second_pass = ALL_LINKS.replace_all(&first_pass, "$link_text");

    // Return result
    second_pass.into()
}
