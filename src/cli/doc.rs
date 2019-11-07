//! Handles printing bighelp pages

use crossterm::{
    cursor::Hide,
    cursor::Show,
    input::{input, InputEvent::*, KeyEvent::*},
    queue,
    screen::{EnterAlternateScreen, LeaveAlternateScreen, RawScreen},
    style::Attribute::*,
    style::Color::*,
};
use std::io::{stdout, Write};
use termimad::*;

/// Render the document
pub(crate) fn run(document: &str) -> anyhow::Result<()> {
    // Create a doc skin
    let mut skin = MadSkin::default();
    skin.set_headers_fg(Yellow);
    skin.bold.set_fg(Magenta);
    skin.italic.add_attr(Underlined);

    // If this is a tty
    if atty::is(atty::Stream::Stdout) {
        // Switch to the Pager Screen
        let mut w = stdout();
        queue!(w, EnterAlternateScreen)?;
        let _raw = RawScreen::into_raw_mode()?;
        queue!(w, Hide)?;

        // Create a scrollable area for the markdown renderer
        let mut area = Area::full_screen();
        area.pad(1, 1);
        let mut view = MadView::from(document.to_owned(), area, skin);

        // Listen for events and redraw screen
        let mut events = input().read_sync();
        loop {
            view.write_on(&mut w)?;
            if let Some(Keyboard(key)) = events.next() {
                match key {
                    Up | Char('k') => view.try_scroll_lines(-1),
                    Down | Char('j') => view.try_scroll_lines(1),
                    PageUp => view.try_scroll_pages(-1),
                    PageDown => view.try_scroll_pages(1),
                    Esc | Enter | Char('q') => break,
                    _ => (),
                }
                // Make it full-screen in case the terminal size has changed.
                // For now it will only resize if you hit a button
                view.resize(&Area::full_screen());
                w.flush()?;
            }
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

use clap::{App, AppSettings};

/// Return the `doc` subcommand
pub(crate) fn get_subcommand<'a>() -> App<'a> {
    crate::cli::new_app("doc")
        .about("Show a detailed help page ( like a man page )")
        .setting(AppSettings::DisableHelpSubcommand)
        .unset_setting(AppSettings::ArgRequiredElseHelp)
}
