//! Handles printing bighelp pages

use crossterm::{
    cursor::Hide,
    cursor::Show,
    input::{input, InputEvent::*, KeyEvent::*},
    queue,
    screen::{EnterAlternateScreen, LeaveAlternateScreen, RawScreen},
    style::Color::*,
    style::Attribute::*,
};
use std::io::{stdout, Write};
use termimad::*;

pub(crate) fn help(args: &clap::ArgMatches, page: &str) {
    if args.is_present("show_bighelp") {
        // Customize doc style
        let mut skin = MadSkin::default();
        skin.set_headers_fg(Yellow);
        skin.bold.set_fg(Magenta);
        skin.italic.add_attr(Underlined);
        
        // If this is a tty
        if atty::is(atty::Stream::Stdout) {
            // Switch to the Pager Screen
            let mut w = stdout();
            queue!(w, EnterAlternateScreen).unwrap();
            let _raw = RawScreen::into_raw_mode().unwrap();
            queue!(w, Hide).unwrap();

            // Create a scrollable area for the markdown renderer
            let mut area = Area::full_screen();
            area.pad(1, 1);
            let mut view = MadView::from(page.to_owned(), area, skin);

            // Listen for events and redraw screen
            let mut events = input().read_sync();
            loop {
                view.write_on(&mut w).unwrap();
                if let Some(Keyboard(key)) = events.next() {
                    match key {
                        Up | Char('k') => view.try_scroll_lines(-1),
                        Down | Char('j') => view.try_scroll_lines(1),
                        PageUp => view.try_scroll_pages(-1),
                        PageDown => view.try_scroll_pages(1),
                        Esc | Char('q') => break,
                        _ => (),
                    }
                    w.flush().unwrap();
                }
            }

            // Clean up revert screen
            queue!(w, Show).unwrap();
            queue!(w, LeaveAlternateScreen).unwrap();
            w.flush().unwrap();

        // If this isn't a tty
        } else {
            // Print page
            // NOTE: This will still print out the colors so that you can pipe
            // the output to `less -R` or `cat` and still get the color.
            skin.print_text(&page);
        }

        // Exit process
        std::process::exit(0);
    }
}

pub(crate) fn arg<'a, 'b>() -> clap::Arg<'a, 'b> {
    clap::Arg::with_name("show_bighelp")
        .help("Show a detailed help page ( like a man page )")
        .long("bighelp")
        .short("H")
}