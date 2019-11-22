use anyhow::Context;
use clap::{App, ArgMatches};

mod set_status;

use crate::cli::doc;

#[rustfmt::skip]
/// Return the `client` subcommand
pub(crate) fn get_subcommand<'a>() -> App<'a> {
    crate::cli::new_app("client")
        .about("Communicate with the Lucky daemon in charm scripts")
        .subcommand(set_status::get_subcommand())
        .arg(doc::get_arg())
        // TODO: These are the exact same arguments as for the `daemon` subcommand:
        // we should probably remove the code duplication.
        .args(&crate::cli::get_daemon_connection_args())
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

    crate::log::init_default_logger()?;

    // Run a subcommand
    match args.subcommand() {
        ("set-status", Some(sub_args)) => set_status::run(sub_args).context("Could not set status"),
        _ => panic!("Unimplemented subcommand or failure to show help."),
    }
}
