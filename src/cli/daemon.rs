use anyhow::Context;
use clap::{App, ArgMatches};

mod start;
mod trigger_hook;

use crate::cli::doc;

/// Return the `daemon` subcommand
pub(crate) fn get_subcommand<'a>() -> App<'a> {
    crate::cli::new_app("daemon")
        .about("Run the Lucky daemon")
        .subcommand(start::get_subcommand())
        .subcommand(trigger_hook::get_subcommand())
        .arg(doc::get_arg())
}

/// Run the `daemon` subcommand
pub(crate) fn run(args: &ArgMatches) -> anyhow::Result<()> {
    // Show the docs if necessary
    doc::show_doc(
        &args,
        get_subcommand(),
        "lucky_daemon",
        include_str!("daemon/daemon.md"),
    )?;

    // Run a subcommand
    match args.subcommand() {
        ("start", Some(sub_args)) => start::run(sub_args).context("Could not start daemon"),
        ("trigger-hook", Some(sub_args)) => {
            trigger_hook::run(sub_args).context("Could not run hook")
        }
        _ => panic!("Unimplemented subcommand or failure to show help."),
    }
}
