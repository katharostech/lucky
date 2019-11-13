use clap::{App, ArgMatches};

use crate::cli::doc;

/// Return the `start` subcommand
pub(crate) fn get_subcommand<'a>() -> App<'a> {
    crate::cli::new_app("start")
        .unset_setting(clap::AppSettings::ArgRequiredElseHelp)
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

    let service = crate::rpc::get_service();

    varlink::listen(
        service,
        "unix:/run/lucky.sock;mode=700",
        1, // Min worker threads
        num_cpus::get(), // Max worker threads
        0, // Timeout
    )?;

    Ok(())
}
