use anyhow::Context;
use clap::{App, Arg, ArgMatches};

use std::io::Write;

use crate::cli::*;
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
            .arg(Arg::with_name("host_port")
                .help("The port to bind on the host"))
            .arg(Arg::with_name("container_port")
                .help("The port to bind to in the container"))
            .arg(Arg::with_name("protocol")
                .help("The protocol to bind")
                .possible_values(&["tcp", "udp"])
                .default_value("tcp"))
            .arg(super::container_arg())
    }

    fn get_subcommands(&self) -> Vec<Box<dyn CliCommand<'a>>> {
        vec![]
    }

    fn get_doc(&self) -> Option<CliDoc> {
        None
    }

    fn execute_command(&self, args: &ArgMatches, mut data: CliData) -> anyhow::Result<CliData> {
        let host_port = args
            .value_of("host_port")
            .expect("Missing required arg: host-port");
        let container_port = args
            .value_of("container_port")
            .expect("Missing required arg: container-port");
        let protocol = args
            .value_of("protocol")
            .expect("Missing required arg: protocol");
        let container = args.value_of("container");

        // Get client connection
        let mut client: Box<VarlinkClient> = data
            .remove("client")
            .expect("Missing client data")
            .downcast()
            .expect("Invalid type");

        client
            .container_port_add(
                host_port.parse().context("Could not parse host-port")?,
                container_port
                    .parse()
                    .context("Could not parse container-port")?,
                protocol.into(),
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
            .arg(Arg::with_name("host_port")
                .help("The port to bind on the host")
                .required_unless("all"))
            .arg(Arg::with_name("container_port")
                .help("The port to bind to in the container")
                .required_unless("all"))
            .arg(Arg::with_name("protocol")
                .help("The protocol to bind")
                .possible_values(&["tcp", "udp"])
                .default_value("tcp")
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
            let host_port = args
                .value_of("host_port")
                .expect("Missing required arg: host-port");
            let container_port = args
                .value_of("container_port")
                .expect("Missing required arg: container-port");
            let protocol = args
                .value_of("protocol")
                .expect("Missing required arg: protocol");

            client
                .container_port_remove(
                    host_port.parse().context("Could not parse host-port")?,
                    container_port
                        .parse()
                        .context("Could not parse container-port")?,
                    protocol.into(),
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
