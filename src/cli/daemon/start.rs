use anyhow::Context;
use clap::{App, Arg, ArgMatches};

use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::process::{Command, Stdio};

use crate::cli::doc;

#[rustfmt::skip]
/// Return the `start` subcommand
pub(crate) fn get_subcommand<'a>() -> App<'a> {
    crate::cli::new_app("start")
        .unset_setting(clap::AppSettings::ArgRequiredElseHelp)
        .about("Start the Lucky daemon")
        .arg(doc::get_arg())
        .arg(Arg::with_name("foreground")
            .long("foreground")
            .short('F')
            .help("Run in the foreground"))
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

    // If we are running in the forground
    if args.is_present("foreground") {
        // Start varlink server
        varlink::listen(
            service,
            &listen_address,
            running.clone(),
            1,               // Min worker threads
            num_cpus::get(), // Max worker threads
            0,               // Timeout
        )?;
    } else {
        // Spawn another process for running the daemon in the background
        Command::new(std::env::current_exe()?)
            .args(&["daemon", "--socket-path", &socket_path, "start", "-F"])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .context("Could not start lucky daemon")?;
    }

    Ok(())
}
