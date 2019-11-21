use anyhow::Context;
use clap::{App, AppSettings, ArgMatches};

mod start;
mod stop;
mod trigger_hook;

use crate::cli::doc;
use std::sync::{Arc, RwLock};

#[rustfmt::skip]
/// Return the `daemon` subcommand
pub(crate) fn get_subcommand<'a>() -> App<'a> {
    crate::cli::new_app("daemon")
        .about("Run the Lucky daemon")
        .subcommand(start::get_subcommand())
        .subcommand(trigger_hook::get_subcommand())
        .subcommand(stop::get_subcommand())
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .arg(doc::get_arg())
        .args(&crate::cli::get_daemon_connection_args())
}

use crate::log::DaemonLogger;
static DAEMON_LOGGER: DaemonLogger = DaemonLogger;

/// Run the `daemon` subcommand
pub(crate) fn run(args: &ArgMatches) -> anyhow::Result<()> {
    // Show the docs if necessary
    doc::show_doc(
        &args,
        get_subcommand(),
        "lucky_daemon",
        include_str!("daemon/daemon.md"),
    )?;

    // Enable logging
    log::set_logger(&DAEMON_LOGGER)
        .map(|()| {
            log::set_max_level(log::LevelFilter::Debug);
        })
        .map_err(|e| anyhow::anyhow!("Could not set logger: {}", e))?;

    // Determine the socket path
    let socket_path = match args.value_of("socket_path") {
        Some(path) => path.to_string(),
        None => format!(
            "/run/lucky_{}.sock",
            args.value_of("unit_name")
                .expect("Missing required argument: unit_name")
                .replace("/", "_")
        ),
    };

    // Run a subcommand
    let result = match args.subcommand() {
        ("start", Some(sub_args)) => {
            start::run(sub_args, &socket_path).context("Could not start daemon")
        }
        ("trigger-hook", Some(sub_args)) => {
            trigger_hook::run(sub_args, &socket_path).context("Could not trigger hook")
        }
        ("stop", Some(sub_args)) => {
            stop::run(sub_args, &socket_path).context("Could not stop daemon")
        }
        _ => panic!("Unimplemented subcommand or failure to show help."),
    };

    // Log errors and exit
    if let Err(e) = result {
        log::error!("{:?}", e);
        std::process::exit(1);
    } else {
        Ok(())
    }
}

/// Test connection to daemon
///
/// Returns true if it can connect successfully to daemon
pub(crate) fn can_connect_daemon(connection_address: &str) -> bool {
    varlink::Connection::with_address(connection_address).is_ok()
}

/// Try to connect to daemon with 4 retries and 500 milisecond wait in-between
pub(crate) fn try_connect_daemon(
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
