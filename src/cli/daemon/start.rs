use clap::{App, ArgMatches};

use crate::cli::doc;

/// Return the `start` subcommand
pub(crate) fn get_subcommand<'a>() -> App<'a> {
    crate::cli::new_app("start")
        .about("Start the Lucky daemon")
        .arg(doc::get_arg())
}

/// Run the `start` subcommand
pub(crate) fn run(args: &ArgMatches) -> anyhow::Result<()> {
    // Show the docs if necessary
    doc::show_doc(
        &args,
        get_subcommand(),
        "lucky_daemon_start",
        include_str!("start/start.md"),
    )?;

    println!("TODO: Implement `lucky daemon start`");

    Ok(())
}
