use anyhow::Context;
use clap::{App, Arg, ArgMatches};

use std::process::{Command, Stdio};
use std::sync::{
    mpsc::sync_channel,
    atomic::{AtomicBool, Ordering},
    Arc,
};

use crate::cli::daemon::{can_connect_daemon, try_connect_daemon};
use crate::cli::doc;

#[rustfmt::skip]
/// Return the `start` subcommand
pub(crate) fn get_subcommand<'a>() -> App<'a> {
    crate::cli::new_app("start")
        .unset_setting(clap::AppSettings::ArgRequiredElseHelp)
        .about("Start the Lucky daemon")
        .arg(doc::get_arg())
        .arg(Arg::with_name("ignore_already_running")
            .long("ignore-already-running")
            .short('i')
            .help("Don't complain if the daemon is already running"))
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

    // The stop_listening flag is used to shutdown the server by setting it to `false`
    let stop_listening = Arc::new(AtomicBool::new(false));
    // Get daemon service
    let service = crate::daemon::get_service(stop_listening.clone());
    let listen_address = format!("unix:{};mode=700", socket_path);

    // Set signal handler for SIGINT/SIGTERM
    let stop = stop_listening.clone();
    ctrlc::set_handler(move || {
        log::info!("Shutting down server");
        stop.store(true, Ordering::Relaxed);
    })
    .context("Error setting signal handler for SIGINT/SIGTERM")?;

    // Make sure a daemon is not already running
    if can_connect_daemon(&listen_address) {
        if args.is_present("ignore_already_running") {
            std::process::exit(0);
        } else {
            anyhow::bail!("Daemon is already running");
        }
    }

    // If we are running in the forground
    if args.is_present("foreground") {
        log::info!("Starting daemon in foreground");

        // Start varlink server in its own thread
        let (sender, reciever) = sync_channel(0);
        let thread = std::thread::spawn(move ||{
            sender.send(varlink::listen(
                service,
                &listen_address,
                &varlink::ListenConfig {
                    max_worker_threads: num_cpus::get(),
                    stop_listening: Some(stop_listening.clone()),
                    ..Default::default()
                }
            )).expect("Could not send result over thread");
        });
        // Get the server start resut and wait for the thread to exit
        reciever.recv().expect("Could not recieve result from thread")?;
        thread.join().expect("Could not join to thread");

    // If we should start in background
    } else {
        log::info!("Starting the lucky daemon");

        // Spawn another process for running the daemon in the background
        Command::new(std::env::current_exe()?)
            .args(&["daemon", "--socket-path", &socket_path, "start", "-F"])
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .context("Could not start lucky daemon")?;

        // Make sure we can connect to the daemon
        try_connect_daemon(&listen_address)
            .and_then(|_| {
                log::info!("Daemon started");
                Ok(())
            })
            // If we can't connect to the daemon
            .or_else(move |_| {
                Err(anyhow::anyhow!(concat!(
                    "Could not start daemon. Try running in the foreground with the -F flag to ",
                    "see what the error message is."
                )))
            })?;
    }

    Ok(())
}
