use clap::{App, AppSettings, ArgMatches};

use std::collections::HashMap;

// Subcommands
mod container;
mod get_config;
mod get_resource;
mod kv;
mod leader;
mod port;
mod private_address;
mod public_address;
mod random;
mod relation;
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
            .about("Communicate with Lucky and Juju in charm scripts");

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
            Box::new(port::PortSubcommand),
            Box::new(relation::RelationSubcommand),
            Box::new(leader::LeaderSubcommand),
            Box::new(random::RandomSubcommand),
            Box::new(get_resource::GetResourceSubcommand),
        ]
    }

    fn get_doc(&self) -> Option<CliDoc> {
        Some(CliDoc {
            name: "lucky_client",
            content: include_str!("cli_help/lucky_client.md"),
        })
    }

    // Client commands are only available when Lucky is built with the "daemon" feature
    fn only_with_daemon(&self) -> bool {
        true
    }

    #[cfg(feature = "daemon")]
    fn execute_command(&self, args: &ArgMatches, mut data: CliData) -> anyhow::Result<CliData> {
        // Skip creation of client data if the matched subcommand was "random", which doesn't need
        // the client connection.
        if args.subcommand_matches("random").is_some() {
            return Ok(data);
        }

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
            "JUJU_REMOTE_APP",
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
