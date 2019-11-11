// use anyhow::Context;
use clap::{App, ArgMatches};

// mod start;
// mod run_hook;

use crate::cli::doc;

/// Return the `client` subcommand
pub(crate) fn get_subcommand<'a>() -> App<'a> {
    crate::cli::new_app("client")
        .about("Communicate with the Lucky daemon")
        // .subcommand(start::get_subcommand())
        // .subcommand(run_hook::get_subcommand())
        .arg(doc::get_arg())
}

/// Run the `client` subcommand
pub(crate) fn run(args: &ArgMatches) -> anyhow::Result<()> {
    // Show the docs if necessary
    doc::show_doc(
        &args,
        get_subcommand(),
        "lucky_clent",
        include_str!("client/client.md"),
    )?;

    // Run a subcommand
    match args.subcommand() {
        // ("start", Some(sub_args)) => start::run(sub_args).context("Could not start daemon"),
        // ("run_hook", Some(sub_args)) => run_hook::run(sub_args).context("Could not run hook"),
        _ => panic!("Unimplemented subcommand or failure to show help."),
    }
}
