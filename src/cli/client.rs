use clap::{App, ArgMatches};

use std::collections::HashMap;

// Subcommands
mod container;
mod kv;
mod set_status;

use crate::cli::daemon::get_daemon_client;
use crate::cli::daemon::{get_daemon_connection_args, get_daemon_socket_path};
use crate::cli::*;

pub(super) struct ClientSubcommand;

impl<'a> CliCommand<'a> for ClientSubcommand {
    fn get_name(&self) -> &'static str {
        "client"
    }

    fn get_app(&self) -> App<'a> {
        self.get_base_app()
            .about("Communicate with the Lucky daemon in charm scripts")
            // TODO: FIXME: Causes issue where client subcommands --doc flag will not work because
            // of required `unit_name` arg.
            .args(&get_daemon_connection_args())
    }

    fn get_subcommands(&self) -> Vec<Box<dyn CliCommand<'a>>> {
        vec![
            Box::new(set_status::SetStatusSubcommand),
            Box::new(kv::KvSubcommand),
            Box::new(container::ContainerSubcommand),
        ]
    }

    fn get_doc(&self) -> Option<CliDoc> {
        None
    }

    fn execute_command(&self, args: &ArgMatches, mut data: CliData) -> anyhow::Result<CliData> {
        let socket_path = get_daemon_socket_path(args);

        // Connect to lucky daemon
        let client = get_daemon_client(&socket_path)?;

        // Add client to data for use in subcommands
        data.insert("client".into(), Box::new(client));

        // Get environment variables that the daemon may need from client
        let mut environment = HashMap::<String, String>::new();
        for &var in &[
            "JUJU_RELATION",
            "JUJU_RELATION_ID",
            "JUJU_REMOTE_UNIT",
            "JUJU_CONTEXT_ID",
        ] {
            if let Ok(value) = std::env::var(var) {
                environment.insert(var.into(), value);
            }
        }

        // Add environment to data for use in subcommands
        data.insert("environment".into(), Box::new(environment));

        Ok(data)
    }
}
