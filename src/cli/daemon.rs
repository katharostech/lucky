use anyhow::Context;
use clap::{App, Arg, ArgMatches};

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
        .arg(doc::get_arg())
        .args(&crate::cli::daemon::get_daemon_connection_args())
}

/// Run the `daemon` subcommand
pub(crate) fn run(args: &ArgMatches) -> anyhow::Result<()> {
    // Show the docs if necessary
    doc::show_doc(
        &args,
        get_subcommand(),
        "lucky_daemon",
        include_str!("daemon/daemon.md"),
    )?;

    crate::log::init_daemon_logger()?;

    // Determine the socket path
    let socket_path = get_daemon_socket_path(&args);

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
        _ => get_subcommand()
            .write_help(&mut std::io::stderr())
            .map_err(|e| e.into()),
    };

    // Log errors and exit
    if let Err(e) = result {
        log::error!("{:?}", e);
        std::process::exit(1);
    } else {
        Ok(())
    }
}

//
// Helpers
//

/// Returns the set of arguments required for any command connecting to the daemon.
pub(crate) fn get_daemon_connection_args<'a>() -> [Arg<'a>; 2] {
    [
        Arg::with_name("unit_name")
            .long("unit-name")
            .short('u')
            .help("The name of the Juju unit that this daemon is running for")
            .long_help(concat!(
                "The name of the Juju unit that this daemon is running for. This will be used to ",
                "determine path to the socket to listen on. For example a unit name of ",
                r#""mysql/2" would listen on the socket "/run/lucky_mysql_2.sock"."#
            ))
            .takes_value(true)
            .env("JUJU_UNIT_NAME")
            .required_unless("socket_path"),
        Arg::with_name("socket_path")
            .long("socket-path")
            .short('s')
            .help("The path to the socket to listen on")
            .long_help(concat!(
                "The path to the socket to listen on. This will override the path determined by ",
                "the unit-name argument."
            ))
            .takes_value(true)
            .required_unless("unit_name")
            .env("LUCKY_DAEMON_SOCKET"),
    ]
}

/// Get the effective socket path from the daemon connection args provided by
/// `get_daemon_connection_args`.
pub(crate) fn get_daemon_socket_path(args: &ArgMatches) -> String {
    match args.value_of("socket_path") {
        Some(path) => path.to_string(),
        None => format!(
            "/run/lucky_{}.sock",
            args.value_of("unit_name")
                .expect("Missing required argument: unit_name")
                .replace("/", "_")
        ),
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
    socket_path: &str,
) -> anyhow::Result<Arc<RwLock<varlink::Connection>>> {
    let connection_address = format!("unix:{}", &socket_path);

    let mut retries = 0;
    let max_retries = 4;
    let result;
    loop {
        let connection_result = varlink::Connection::with_address(&connection_address);
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
