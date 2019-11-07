//! Commandline interface module

use clap::{App, AppSettings};

// Help utility
pub(crate) mod doc;

// Subcommands
mod charm;

/// Run the application
pub fn run() {
    // Enable colored backtraces
    #[cfg(feature = "color-backtrace")]
    color_backtrace::install();

    // Collect arguments from the commandline
    let args = get_cli().get_matches();

    // Run the chosen subcommand
    if let Err(e) = match args.subcommand() {
        ("charm", Some(sub_args)) => charm::run(sub_args),
        ("doc", _) => doc::run(include_str!("cli/lucky.md")),
        _ => panic!("Unimplemented subcommand or failure to show help."),
    } {
        // If this fails to print we don't care to handle the error, we can't do anything about it
        // but we don't want to panic.
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

/// Returns a default app with the given name. This is used by subcommands to provide
/// modifiable default settings.
fn new_app<'a>(name: &str) -> App<'a> {
    App::new(name)
        .setting(AppSettings::ColoredHelp)
        .setting(AppSettings::VersionlessSubcommands)
        .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::DisableHelpSubcommand)
        .mut_arg("help", |arg| {
            arg.short('h')
                .long("help")
                .help("Show help: --help shows more information when available")
        })
}

/// Get the Lucky clap App
fn get_cli() -> App<'static> {
    new_app("lucky")
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about("The Lucky charm framework for Juju.")
        .subcommand(charm::get_subcommand())
        .subcommand(doc::get_subcommand())
}
