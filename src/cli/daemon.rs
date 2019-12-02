use anyhow::Context;
use clap::{App, Arg, ArgMatches};

mod start;
mod stop;
mod trigger_hook;

use crate::cli::*;
use std::sync::{Arc, RwLock};

pub(crate) struct DaemonSubcommand;

impl<'a> CliCommand<'a> for DaemonSubcommand {
    fn get_name(&self) -> &'static str {
        "daemon"
    }

    fn get_command(&self) -> App<'a> {
        self.get_base_app()
            .about("Start, stop, and trigger the Lucky daemon")
    }

    fn get_subcommands(&self) -> Vec<Box<dyn CliCommand<'a>>> {
        vec![
            Box::new(start::StartSubcommand),
            Box::new(stop::StopSubcommand),
            Box::new(trigger_hook::TriggerHookSubcommand),
        ]
    }

    fn get_doc(&self) -> Option<CliDoc> {
        None
    }

    fn execute_command(&self, _args: &ArgMatches) -> anyhow::Result<()> {
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
            .required_unless("doc"),
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
