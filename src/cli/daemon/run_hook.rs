use clap::{App, ArgMatches};

use crate::cli::doc;

/// Return the `run-hook` subcommand
pub(crate) fn get_subcommand<'a>() -> App<'a> {
    crate::cli::new_app("run-hook")
        .about("Run a hook through the Lucky daemon")
        .arg(doc::get_arg())
}

/// Run the `run-hook` subcommand
pub(crate) fn run(args: &ArgMatches) -> anyhow::Result<()> {
    // Show the docs if necessary
    doc::show_doc(
        &args,
        get_subcommand(),
        "lucky_daemon_run-hook",
        include_str!("run_hook/run_hook.md"),
    )?;

    println!("TODO: Implement `lucky daemon run-hook`");

    Ok(())
}
