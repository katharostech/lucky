use anyhow::Context;
use clap::{App, ArgMatches};

use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use crate::cli::doc;

#[rustfmt::skip]
/// Return the `start` subcommand
pub(crate) fn get_subcommand<'a>() -> App<'a> {
    crate::cli::new_app("start")
        .unset_setting(clap::AppSettings::ArgRequiredElseHelp)
        .about("Start the Lucky daemon")
        .arg(doc::get_arg())
}

/// Run the `start` subcommand
pub(crate) fn run(args: &ArgMatches, socket_path: &str) -> anyhow::Result<()> {
    // Show the docs if necessary
    doc::show_doc(
        &args,
        get_subcommand(),
        "lucky_daemon_start",
        include_str!("start/start.md"),
    )?;

    // The running flag is used to shutdown the server by setting it to `false`
    let running = Arc::new(AtomicBool::new(true));

    let service = crate::daemon::get_service(running.clone());
    let listen_address = format!("unix:{};mode=700", socket_path);

    // Set signal handler
    let r = running.clone();
    ctrlc::set_handler(move || {
        println!("Shutting down server"); // TODO: print this message with logging instead
        r.store(false, Ordering::SeqCst);
    })
    .context("Error setting signal handler for SIGINT/SIGTERM")?;

    // Start varlink server
    varlink::listen(
        service,
        &listen_address,
        running.clone(),
        1,               // Min worker threads
        num_cpus::get(), // Max worker threads
        0,               // Timeout
    )?;

    Ok(())
}
