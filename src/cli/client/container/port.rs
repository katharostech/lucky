use anyhow::Context;
use clap::{App, Arg, ArgMatches};

use std::io::Write;

use crate::cli::*;
#[cfg(feature = "daemon")]
use crate::docker::PortBinding;
use crate::rpc::{VarlinkClient, VarlinkClientInterface};

pub(super) struct PortSubcommand;

impl<'a> CliCommand<'a> for PortSubcommand {
    fn get_name(&self) -> &'static str {
        "port"
    }

    #[rustfmt::skip]
    fn get_app(&self) -> App<'a> {
        self.get_base_app()
            .about("Add and remove container port bindings")
    }

    fn get_subcommands(&self) -> Vec<Box<dyn CliCommand<'a>>> {
        vec![
            Box::new(AddSubcommand),
            Box::new(RemoveSubcommand),
            Box::new(GetSubcommand),
        ]
    }

    fn get_doc(&self) -> Option<CliDoc> {
        None
    }

    fn execute_command(&self, _args: &ArgMatches, data: CliData) -> anyhow::Result<CliData> {
        Ok(data)
    }
}

struct AddSubcommand;

impl<'a> CliCommand<'a> for AddSubcommand {
    fn get_name(&self) -> &'static str {
        "add"
    }

    #[rustfmt::skip]
    fn get_app(&self) -> App<'a> {
        self.get_base_app()
            .unset_setting(clap::AppSettings::ArgRequiredElseHelp)
            .about("Add a port binding")
            .arg(Arg::with_name("port_binding")
                .help("The port binding to add in the format: `host_port:container_port/proto`.")
                .long_help(concat!(
                    "The port binding to add in the format: `host_port:container_port/proto`. ",
                    "the `/proto` suffix is optional and defaults to `/tcp`."
                )))
            .arg(super::container_arg())
    }

    fn get_subcommands(&self) -> Vec<Box<dyn CliCommand<'a>>> {
        vec![]
    }

    fn get_doc(&self) -> Option<CliDoc> {
        None
    }

    #[cfg(not(feature = "daemon"))]
    fn execute_command(&self, _args: &ArgMatches, mut data: CliData) -> anyhow::Result<CliData> {
        Ok(data)
    }

    #[cfg(feature = "daemon")]
    fn execute_command(&self, args: &ArgMatches, mut data: CliData) -> anyhow::Result<CliData> {
        let container = args.value_of("container");

        // Get client connection
        let mut client: Box<VarlinkClient> = data
            .remove("client")
            .expect("Missing client data")
            .downcast()
            .expect("Invalid type");

        let port_binding = args
            .value_of("port_binding")
            .expect("Missing required argument: port");
        let port_binding = port_binding
            .parse::<PortBinding>()
            .context("Could not parse port binding")?;

        client
            .container_port_add(
                port_binding.host_port.into(),
                port_binding.container_port.into(),
                port_binding.protocol,
                container.map(Into::into),
            )
            .call()?;

        Ok(data)
    }
}

struct RemoveSubcommand;

impl<'a> CliCommand<'a> for RemoveSubcommand {
    fn get_name(&self) -> &'static str {
        "remove"
    }

    #[rustfmt::skip]
    fn get_app(&self) -> App<'a> {
        self.get_base_app()
            .about("Remove a port binding")
            .arg(Arg::with_name("port_binding")
                .help("The port binding to remove in the format: `host_port:container_port/proto`.")
                .long_help(concat!(
                    "The port binding to remove in the format: `host_port:container_port/proto`. ",
                    "the `/proto` suffix is optional and defaults to `/tcp`."
                ))
                .required_unless("all"))
            .arg(Arg::with_name("all")
                .help("Remove all port bindings from the container")
                .long("all")
                .short('A'))
            .arg(super::container_arg())
    }

    fn get_subcommands(&self) -> Vec<Box<dyn CliCommand<'a>>> {
        vec![]
    }

    fn get_doc(&self) -> Option<CliDoc> {
        None
    }

    #[cfg(not(feature = "daemon"))]
    fn execute_command(&self, _args: &ArgMatches, mut data: CliData) -> anyhow::Result<CliData> {
        Ok(data)
    }

    #[cfg(feature = "daemon")]
    fn execute_command(&self, args: &ArgMatches, mut data: CliData) -> anyhow::Result<CliData> {
        let remove_all = args.is_present("all");
        let container = args.value_of("container");

        // Get client connection
        let mut client: Box<VarlinkClient> = data
            .remove("client")
            .expect("Missing client data")
            .downcast()
            .expect("Invalid type");

        if remove_all {
            client
                .container_port_remove_all(container.map(Into::into))
                .call()?;
        } else {
            let port_binding = args
                .value_of("port_binding")
                .expect("Missing required argument: port");
            let port_binding = port_binding
                .parse::<PortBinding>()
                .context("Could not parse port binding")?;

            client
                .container_port_remove(
                    port_binding.host_port.into(),
                    port_binding.container_port.into(),
                    port_binding.protocol,
                    container.map(Into::into),
                )
                .call()?;
        }

        Ok(data)
    }
}

struct GetSubcommand;

impl<'a> CliCommand<'a> for GetSubcommand {
    fn get_name(&self) -> &'static str {
        "get"
    }

    #[rustfmt::skip]
    fn get_app(&self) -> App<'a> {
        self.get_base_app()
            .about("Get a list of the containers port bindings")
            .arg(super::container_arg())
    }

    fn get_subcommands(&self) -> Vec<Box<dyn CliCommand<'a>>> {
        vec![]
    }

    fn get_doc(&self) -> Option<CliDoc> {
        None
    }

    #[cfg(not(feature = "daemon"))]
    fn execute_command(&self, _args: &ArgMatches, mut data: CliData) -> anyhow::Result<CliData> {
        Ok(data)
    }

    #[cfg(feature = "daemon")]
    fn execute_command(&self, args: &ArgMatches, mut data: CliData) -> anyhow::Result<CliData> {
        let container = args.value_of("container");

        // Get client connection
        let mut client: Box<VarlinkClient> = data
            .remove("client")
            .expect("Missing client data")
            .downcast()
            .expect("Invalid type");

        for port_binding in client
            .container_port_get_all(container.map(Into::into))
            .call()?
            .ports
        {
            writeln!(
                std::io::stdout(),
                "{}:{}/{}",
                port_binding.host_port,
                port_binding.container_port,
                port_binding.protocol
            )?;
        }

        Ok(data)
    }
}
