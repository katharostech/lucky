//! Commandline interface module

use clap::{App, AppSettings, Arg};
use crossterm::style::{style, Color};

use std::io::Write;

// Help utility
pub(crate) mod doc;

// Subcommands
mod charm;
mod client;
mod daemon;

/// Run the application
pub fn run() {
    std::panic::catch_unwind(|| {
        // run program and report any errors
        if let Err(e) = execute() {
            writeln!(
                std::io::stderr(),
                "\n{} {:?}",
                style("Error:").with(Color::Red),
                e
            )
            .ok();
            std::process::exit(1);
        }
    })
    // Catch any panics and print an error message. This will appear after the message given by
    // colored backtrace.
    .or_else(|_| -> Result<(), ()> {
        writeln!(
            std::io::stderr(),
            concat!(
                "\n {} The program has encountered a critical internal error and will now exit.\n",
                "This is a bug. TODO: Setup Taiga project for reporting errors!!\n"
            ),
            style("Error:").with(Color::Red)
        )
        .ok();

        Ok(())
    })
    .expect("Panic while handling panic");
}

fn execute() -> anyhow::Result<()> {
    // Enable colored backtraces
    #[cfg(feature = "color-backtrace")]
    color_backtrace::install();

    // Collect arguments from the commandline
    let args = get_cli()?.get_matches();

    // If there is a specified Lucky context
    if let Ok(context) = std::env::var("LUCKY_CONTEXT") {
        // Run the specified subcommand instead
        return match context.as_ref() {
            "charm" => return charm::run(&args),
            "daemon" => return daemon::run(&args),
            "client" => client::run(&args),
            other => anyhow::bail!("Unrecognized LUCKY_CONTEXT: {}", other),
        };
    }

    // Show the docs if necessary
    doc::show_doc(&args, get_cli()?, "lucky", include_str!("cli/cli.md"))?;

    // Run a subcommand
    match args.subcommand() {
        ("charm", Some(sub_args)) => charm::run(sub_args),
        ("daemon", Some(sub_args)) => daemon::run(sub_args),
        ("client", Some(sub_args)) => client::run(sub_args),
        _ => get_cli()?.write_help(&mut std::io::stderr()).map_err(|e| e.into()),
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
                .help("Show help: --help shows more information")
        })
}

/// Returns the set of arguments required for any command connecting to the daemon.
fn get_daemon_connection_args<'a>() -> [Arg<'a>; 2] {
    [
        Arg::with_name("unit_name")
            .long("unit-name")
            .short('u')
            .help("The name of the Juju unit that this daemon is running for")
            .long_help(concat!(
                "The name of the Juju unit that this daemon is running for. This will be used to ",
                "determine path to the socket to listen on. For example a unit name of ",
                r#""mysql/2" would listen on the socket "/run/lucky_mysql_2.sock"."#
            ))
            .takes_value(true)
            .env("JUJU_UNIT_NAME")
            .required_unless("socket_path"),
        Arg::with_name("socket_path")
            .long("socket-path")
            .short('s')
            .help("The path to the socket to listen on")
            .long_help(concat!(
                "The path to the socket to listen on. This will override the path determined by ",
                "the unit-name argument."
            ))
            .takes_value(true)
            .required_unless("unit_name")
            .env("LUCKY_DAEMON_SOCKET"),
    ]
}

/// Get the Lucky clap App
fn get_cli() -> anyhow::Result<App<'static>> {
    // If there is a specified context
    if let Ok(context) = std::env::var("LUCKY_CONTEXT") {
        // Return the specified subcommand instead of the global CLI
        match context.as_ref() {
            "charm" => return Ok(charm::get_subcommand()),
            "daemon" => return Ok(daemon::get_subcommand()),
            "client" => return Ok(client::get_subcommand()),
            other => anyhow::bail!("Unrecognized LUCKY_CONTEXT: {}", other),
        }
    }

    // Return full CLI
    Ok(new_app("lucky")
        .version(clap::crate_version!())
        .about("The Lucky charm framework for Juju.")
        .arg(doc::get_arg())
        .subcommand(charm::get_subcommand())
        .subcommand(daemon::get_subcommand())
        .subcommand(client::get_subcommand()))
}
