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
        .about("Start, stop, and trigger the Lucky daemon")
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

    // Initialize the logger
    crate::log::init_daemon_logger()?;

    let unit_name = args
        .value_of("unit_name")
        .expect("Missing required argument \"unit_name\"");
    // Determine the socket path
    let socket_path = get_daemon_socket_path(&args);

    // Run a subcommand
    let result = match args.subcommand() {
        ("start", Some(sub_args)) => {
            start::run(sub_args, &unit_name, &socket_path).context("Could not start daemon")
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
            .takes_value(true)
            .env("JUJU_UNIT_NAME")
            .required(true),
        Arg::with_name("socket_path")
            .long("socket-path")
            .short('s')
            .help("The path to the socket to listen on")
            .long_help(concat!(
                "The path of the socket to listen on. If this is left unspecified the socket path ",
                "will be automatically determined from the unit name. For example, for a unit ",
                r#"named "mysql/2", the socket path will be "/run/lucky_mysql_2.sock""#
            ))
            .takes_value(true)
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

/// Get the daemon varlink client from the socket path to connect to
pub(crate) fn get_daemon_client(
    socket_path: &str,
) -> anyhow::Result<crate::daemon::rpc::VarlinkClient> {
    let connection = try_connect_daemon(socket_path)?;
    Ok(crate::daemon::get_client(connection))
}
