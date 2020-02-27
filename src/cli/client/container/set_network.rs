use clap::{App, Arg, ArgMatches};

use crate::cli::*;
use crate::rpc::{VarlinkClient, VarlinkClientInterface};

pub(super) struct SetNetworkSubcommand;

impl<'a> CliCommand<'a> for SetNetworkSubcommand {
    fn get_name(&self) -> &'static str {
        "set-network"
    }

    #[rustfmt::skip]
    fn get_app(&self) -> App<'a> {
        self.get_base_app()
            .about("Set the docker network")
            .arg(Arg::with_name("unset")
                .help("Unset the network instead of setting it")
                .long_help(concat!(
                    "Unset the network instead of setting it. The container will use the ",
                    "default bridge network."
                ))
                .long("unset")
                .short('u')
                .required_unless("network"))
            .arg(Arg::with_name("network")
                .help("The network for the container to use")
                .required_unless("unset"))
            .arg(super::container_arg())
    }

    fn get_subcommands(&self) -> Vec<Box<dyn CliCommand<'a>>> {
        vec![]
    }

    fn get_doc(&self) -> Option<CliDoc> {
        Some(CliDoc {
            name: "lucky_client_container_set-network",
            content: include_str!("cli_help/set_network.md"),
        })
    }

    fn execute_command(&self, args: &ArgMatches, mut data: CliData) -> anyhow::Result<CliData> {
        let container = args.value_of("container");
        let network = args.value_of("network");

        // Get client connection
        let mut client: Box<VarlinkClient> = data
            .remove("client")
            .expect("Missing client data")
            .downcast()
            .expect("Invalid type");

        if args.is_present("unset") {
            // Unset the network
            client
                .container_network_set(None, container.map(Into::into))
                .call()?;
        } else {
            // Set the network
            client
                .container_network_set(
                    Some(network.expect("Missing required argument: network").into()),
                    container.map(Into::into),
                )
                .call()?;
        }

        Ok(data)
    }
}
