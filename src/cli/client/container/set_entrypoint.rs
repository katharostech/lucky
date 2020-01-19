use clap::{App, Arg, ArgMatches};

use crate::cli::*;
use crate::rpc::{VarlinkClient, VarlinkClientInterface};

pub(super) struct SetEntrypointSubcommand;

impl<'a> CliCommand<'a> for SetEntrypointSubcommand {
    fn get_name(&self) -> &'static str {
        "set-entrypoint"
    }

    #[rustfmt::skip]
    fn get_app(&self) -> App<'a> {
        self.get_base_app()
            .about("Set the docker entrypoint")
            .arg(Arg::with_name("entrypoint")
                .help("The entrypoint for the container")
                .required_unless("unset"))
            .arg(Arg::with_name("unset")
                .help("Unset the entrypoint instead of setting it")
                .long_help(concat!(
                    "Unset the entrypoint instead of setting it. The container will use the ",
                    "default entrypoint."
                ))
                .long("unset")
                .short('u')
                .required_unless("entrypoint"))
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

        if args.is_present("unset") {
            // Unset the entrypoint
            client
                .container_set_entrypoint(None, container.map(Into::into))
                .call()?;
        } else {
            // Set the entrypoint
            client
                .container_set_entrypoint(
                    Some(
                        args.value_of("entrypoint")
                            .expect("Missing required argument entrypoint")
                            .into(),
                    ),
                    container.map(Into::into),
                )
                .call()?;
        }

        Ok(data)
    }
}
