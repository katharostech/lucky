use anyhow::Context;
use clap::{App, Arg, ArgMatches};

use std::process::{Command, Stdio};

use crate::cli::doc;
use crate::cli::daemon::try_connect_daemon;
use crate::daemon::{self, VarlinkClientInterface};

#[rustfmt::skip]
/// Return the `trigger-hook` subcommand
pub(crate) fn get_subcommand<'a>() -> App<'a> {
    crate::cli::new_app("trigger-hook")
        .about("Run a hook through the Lucky daemon")
        .unset_setting(clap::AppSettings::ArgRequiredElseHelp)
        .arg(doc::get_arg())
        .arg(Arg::with_name("hook_name")
            .help("The name of the hook to trigger")
            .required(true))
        .arg(Arg::with_name("start_if_not_running")
            .long("start-if-not-running")
            .short('s')
            .help("Start the lucky daemon if it is not already running"))
}

/// Run the `trigger-hook` subcommand
pub(crate) fn run(args: &ArgMatches, socket_path: &str) -> anyhow::Result<()> {
    // Show the docs if necessary
    doc::show_doc(
        &args,
        get_subcommand(),
        "lucky_daemon_trigger-hook",
        include_str!("trigger_hook/trigger_hook.md"),
    )?;

    // Connect to lucky daemon
    let connection_address = format!("unix:{}", &socket_path);
    let connection_result = varlink::Connection::with_address(&connection_address);
    let connection = connection_result.or_else(|e| {
        // The connection failed and --start-if-not-running has been specified
        if args.is_present("start_if_not_running") {
            // Start lucky daemon
            println!("Starting lucky daemon"); // TODO: implementing logging for this notification
            Command::new(std::env::current_exe()?)
                .args(&["daemon", "--socket-path", &socket_path, "start"])
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()
                .context("Could not start lucky daemon")?;

            // Attempt connection atain
            try_connect_daemon(&connection_address)

        // If there was no --start-if-not-running flag
        } else {
            Err(e).context(format!(
                r#"Could not connect to lucky daemon at: "{}""#,
                connection_address
            ))
        }
    })?;

    // Connect to service and trigger the hook
    let mut service = daemon::get_client(connection);
    service
        .trigger_hook(
            args.value_of("hook_name")
                .expect("Missing required argument: hook_name")
                .to_string(),
        )
        .call()?;

    println!(
        // TODO: logging
        r#"{} Ran hook "{}""#,
        crossterm::style::style("Success:").with(crossterm::style::Color::Green),
        args.value_of("hook_name")
            .expect("Missing required argument: hook_name")
    );

    Ok(())
}
