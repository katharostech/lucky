//! Commandline interface module

use clap::{App, AppSettings};
use crossterm::style::{style, Color};

// Help utility
pub(crate) mod doc;

// Subcommands
mod charm;

/// Run the application
pub fn run() {
    if let Err(e) = execute() {
        eprintln!("\n{} {:?}", style("Error:").with(Color::Red), e);
        std::process::exit(1);
    }
}

fn execute() -> anyhow::Result<()> {
    // Enable colored backtraces
    #[cfg(feature = "color-backtrace")]
    color_backtrace::install();

    // Collect arguments from the commandline
    let args = get_cli().get_matches();

    // Show the docs if necessary
    doc::show_doc(&args, get_cli(), "lucky", include_str!("cli/cli.md"))?;

    // Run a subcommand
    match args.subcommand() {
        ("charm", Some(sub_args)) => charm::run(sub_args),
        _ => panic!("Unimplemented subcommand or failure to show help."),
    }?;

    Ok(())
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
                .help("Show help: --help shows more information")
        })
}

/// Get the Lucky clap App
fn get_cli() -> App<'static> {
    new_app("lucky")
        .version(clap::crate_version!())
        .about("The Lucky charm framework for Juju.")
        .arg(doc::get_arg())
        .subcommand(charm::get_subcommand())
}
