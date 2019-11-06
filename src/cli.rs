use clap::{App, AppSettings};

// Subcommands
mod charm;

pub fn run() {
    // Enable colored backtraces
    #[cfg(feature = "color-backtrace")]
    color_backtrace::install();

    let args = get_cli().get_matches();

    man::manpage(&args, include_str!("cli/lucky.md"));

    match args.subcommand() {
        ("charm", Some(sub_args)) => charm::run(sub_args),
        ("", None) => println!("TODO: show help"),
        _ => panic!("Unimplemented subcommand or failure to show help."),
    }
}

pub(crate) mod man {
    pub(crate) fn manpage(args: &clap::ArgMatches, page: &str) {
        if args.is_present("show_manpage") {
            termimad::print_text(page);
            std::process::exit(0);
        }
    }

    pub(crate) fn arg<'a, 'b>() -> clap::Arg<'a, 'b> {
        clap::Arg::with_name("show_manpage")
            .help("Show a more detailed manual page")
            .long("man")
            .short("H")
    }
}

fn get_cli() -> App<'static, 'static> {
    let mut app = App::new("Lucky")
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about("The Lucky charm framework for Juju.")
        .global_setting(AppSettings::ColoredHelp)
        .arg(man::arg());

    app = app.subcommand(charm::get_subcommand());

    app
}
