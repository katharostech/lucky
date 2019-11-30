//! Commandline interface module

use clap::{App, AppSettings};

mod types;
use types::*;

// Help utility
pub(crate) mod doc;

// Subcommands
mod charm;
mod client;
mod daemon;

/// Run the application
pub fn run() {
    // Initialize logger
    crate::log::init_logger();

    std::panic::catch_unwind(|| {
        // run program and report any errors
        if let Err(e) = execute() {
            if let Some(cli_error) = e.downcast_ref::<CliError>() {
                match cli_error {
                    CliError::Exit(0) => std::process::exit(0),
                    CliError::Exit(code) => {
                        log::error!("{:?}", e);
                        std::process::exit(*code);
                    }
                }
            } else {
                log::error!("{:?}", e);
                std::process::exit(1);
            }
        }
    })
    // Catch any panics and print an error message. This will appear after the message given by
    // colored backtrace.
    .or_else(|_| -> Result<(), ()> {
        log::error!(concat!(
            "The program has encountered a critical internal error and will now exit.\n",
            "This is a bug. TODO: Setup Taiga project for reporting errors!!\n"
        ));

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
        _ => get_cli()?
            .write_help(&mut std::io::stderr())
            .map_err(|e| e.into()),
    }
}

/// Returns a default app with the given name. This is used by subcommands to provide
/// modifiable default settings.
fn new_app<'a>(name: &str) -> App<'a> {
    App::new(name)
        // Set the max term width the 3 short of  the actual width so that we don't wrap on the
        // help pager. Width is 3 shorter because of 1 char for the scrollbar and 1 char padding on
        // each side.
        .max_term_width(
            crossterm::terminal::size()
                .map(|size| size.0 - 3)
                .unwrap_or(0) as usize,
        )
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
        .version(crate::GIT_VERSION)
        .about("The Lucky charm framework for Juju.")
        .arg(doc::get_arg())
        .subcommand(charm::get_subcommand())
        .subcommand(daemon::get_subcommand())
        .subcommand(client::get_subcommand()))
}
