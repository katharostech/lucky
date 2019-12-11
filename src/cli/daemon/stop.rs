use anyhow::Context;
use clap::{App, Arg, ArgMatches};

use crate::cli::daemon::{get_daemon_connection_args, get_daemon_socket_path};
use crate::cli::*;
use crate::daemon::{self, rpc::VarlinkClientInterface};

pub(crate) struct StopSubcommand;

impl<'a> CliCommand<'a> for StopSubcommand {
    fn get_name(&self) -> &'static str {
        "stop"
    }

    fn get_app(&self) -> App<'a> {
        self.get_base_app()
            .unset_setting(clap::AppSettings::ArgRequiredElseHelp)
            .about("Stop the Lucky daemon")
            .arg(
                Arg::with_name("ignore_already_stopped")
                    .long("ignore-already-stopped")
                    .short('i')
                    .help("Don't complain if the daemon is already stopped"),
            )
            .args(&get_daemon_connection_args())
    }

    fn get_subcommands(&self) -> Vec<Box<dyn CliCommand<'a>>> {
        vec![]
    }

    fn get_doc(&self) -> Option<CliDoc> {
        None
    }

    fn execute_command(&self, args: &ArgMatches, data: CliData) -> anyhow::Result<CliData> {
        let socket_path = get_daemon_socket_path(args);

        // Connect to lucky daemon
        let connection_address = format!("unix:{}", &socket_path);
        let connection = match varlink::Connection::with_address(&connection_address) {
            Ok(conn) => Ok(conn),
            Err(e) => {
                if args.is_present("ignore_already_stopped") {
                    Err(CliError::Exit(0).into())
                } else {
                    Err(e).context(format!(
                        r#"Could not connect to lucky daemon at: "{}""#,
                        connection_address
                    ))
                }
            }
        }?;
        let mut service = daemon::get_client(connection);

        // Stop the daemon
        service.stop_daemon().call()?;

        log::info!("Shutdown server");

        Ok(data)
    }
}
