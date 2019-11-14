use anyhow::Context;
use clap::{App, Arg, ArgMatches};

use std::process::{Command, Stdio};
use std::sync::{Arc, RwLock};

use crate::cli::doc;
use crate::rpc::{self, VarlinkClientInterface};

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
            Err(e.into())
        }
    })?;

    let mut service = rpc::get_client(connection);

    service
        .trigger_hook(
            args.value_of("hook_name")
                .expect("Missing required argument: hook_name")
                .to_string(),
        )
        .call()?;

    println!(
        r#"{} Ran hook "{}"!"#,
        crossterm::style::style("Success:").with(crossterm::style::Color::Green),
        args.value_of("hook_name")
            .expect("Missing required argument: hook_name")
    );

    Ok(())
}

/// Try to connect to daemon with 4 retries and 500 milisecond wait in-between
fn try_connect_daemon(
    connection_address: &str,
) -> anyhow::Result<Arc<RwLock<varlink::Connection>>> {
    let mut retries = 0;
    let max_retries = 4;
    let result;
    loop {
        let connection_result = varlink::Connection::with_address(connection_address);
        match connection_result {
            Ok(conn) => {
                result = Ok(conn);
                break;
            }
            Err(e) => {
                retries += 1;
                if retries == max_retries {
                    result = Err(e);
                    break;
                }
            }
        }

        std::thread::sleep(std::time::Duration::from_millis(500))
    }

    result.context(format!(
        r#"Could not connect to lucky daemon at: "{}""#,
        connection_address
    ))
}
