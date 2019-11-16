use anyhow::Context;
use clap::{App, Arg, ArgMatches};

use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::process::{Command, Stdio};

use crate::cli::doc;
use crate::cli::daemon::{try_connect_daemon, can_connect_daemon};

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
    // Get daemon service
    let service = crate::daemon::get_service(running.clone());
    let listen_address = format!("unix:{};mode=700", socket_path);

    // Set signal handler for SIGINT/SIGTERM
    let r = running.clone();
    ctrlc::set_handler(move || {
        log::info!("Shutting down server"); // TODO: print this message with logging instead
        r.store(false, Ordering::SeqCst);
    })
    .context("Error setting signal handler for SIGINT/SIGTERM")?;

    // Make sure a daemon is not already running
    if can_connect_daemon(&listen_address) {
        anyhow::bail!("Daemon is already running");
    }

    // If we are running in the forground
    if args.is_present("foreground") {
        log::info!("Starting daemon in foreground");
        // Start varlink server
        varlink::listen(
            service,
            &listen_address,
            running.clone(),
            1,               // Min worker threads
            num_cpus::get(), // Max worker threads
            0,               // Timeout
        )
        .context(format!("Could not daemon server on socket: {}", &socket_path))?;

    // If we should start in background
    } else {
        log::info!("Starting the lucky daemon");

        // Spawn another process for running the daemon in the background
        let child_daemon = Command::new(std::env::current_exe()?)
            .args(&["daemon", "--socket-path", &socket_path, "start", "-F"])
            .stdout(Stdio::null())
            .stderr(Stdio::piped())
            .spawn()
            .context("Could not start lucky daemon")?;

        // Make sure we can connect to the daemon
        try_connect_daemon(&listen_address)
        // If we can't connect to the daemon
        .or_else(move |_| {
            {
                // Print stderr from daemon process
                let output = child_daemon.wait_with_output()?;
                log::error!("{}", String::from_utf8_lossy(&output.stderr));
            }
            // Exit because we have already printed the error message
            std::process::exit(1);
            // Rustc wants us to have specified an error output
            #[allow(unreachable_code)]
            Err(anyhow::anyhow!("Unreachble error required by compiler"))
        })?;
    }

    Ok(())
}
