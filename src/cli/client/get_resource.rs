use clap::{App, Arg, ArgMatches};

use std::io::Write;

use crate::cli::*;
use crate::rpc::{VarlinkClient, VarlinkClientInterface};

pub(super) struct GetResourceSubcommand;

impl<'a> CliCommand<'a> for GetResourceSubcommand {
    fn get_name(&self) -> &'static str {
        "get-resource"
    }

    #[rustfmt::skip]
    fn get_app(&self) -> App<'a> {
        self.get_base_app()
            .about("Get the path to a Juju resource")
            .long_about(concat!(
                "Get the path to a Juju resource. NOTE: This path is the path to the resource on ",
                "the host and will not be accessible if called from a container. This may be ",
                "fixed later."
            ))
            .arg(Arg::with_name("resource_name")
                .help("The name of the resource to get as defined in the metadata.yaml file")
                .takes_value(true))
    }

    fn get_subcommands(&self) -> Vec<Box<dyn CliCommand<'a>>> {
        vec![]
    }

    fn get_doc(&self) -> Option<CliDoc> {
        None
    }

    #[allow(clippy::filter_map)]
    fn execute_command(&self, args: &ArgMatches, mut data: CliData) -> anyhow::Result<CliData> {
        let resource_name = args
            .value_of("resource_name")
            .expect("Missing required argument: resource_name");

        // Get client connection
        let mut client: Box<VarlinkClient> = data
            .remove("client")
            .expect("Missing client data")
            .downcast()
            .expect("Invalid type");

        // Print out resource path
        writeln!(
            std::io::stdout(),
            "{}",
            client.get_resource(resource_name.into()).call()?.path
        )?;

        Ok(data)
    }
}
