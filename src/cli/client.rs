use clap::{App, AppSettings, ArgMatches};

use std::collections::HashMap;

// Subcommands
mod container;
mod get_config;
mod kv;
mod private_address;
mod public_address;
mod set_status;

#[cfg(feature = "daemon")]
use crate::cli::daemon::get_daemon_client;
#[cfg(feature = "daemon")]
use crate::cli::daemon::{get_daemon_connection_args, get_daemon_socket_path};
use crate::cli::*;

pub(super) struct ClientSubcommand;

impl<'a> CliCommand<'a> for ClientSubcommand {
    fn get_name(&self) -> &'static str {
        "client"
    }

    fn get_app(&self) -> App<'a> {
        let app = self
            .get_base_app()
            .setting(AppSettings::SubcommandRequiredElseHelp)
            .about("Communicate with the Lucky daemon in charm scripts");

        #[cfg(feature = "daemon")]
        let app = app.args(&get_daemon_connection_args());

        app
    }

    fn get_subcommands(&self) -> Vec<Box<dyn CliCommand<'a>>> {
        vec![
            Box::new(set_status::SetStatusSubcommand),
            Box::new(kv::KvSubcommand),
            Box::new(container::ContainerSubcommand),
            Box::new(public_address::PublicAddressSubcommand),
            Box::new(private_address::PrivateAddressSubcommand),
            Box::new(get_config::GetConfigSubcommand),
        ]
    }

    fn get_doc(&self) -> Option<CliDoc> {
        None
    }

    // Client commands are only available when Lucky is built with the "daemon" feature
    fn only_with_daemon(&self) -> bool {
        true
    }

    #[cfg(feature = "daemon")]
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

    #[cfg(not(feature = "daemon"))]
    /// Do nothing if built without "daemon" feature
    fn execute_command(&self, _args: &ArgMatches, data: CliData) -> anyhow::Result<CliData> {
        Ok(data)
    }
}
