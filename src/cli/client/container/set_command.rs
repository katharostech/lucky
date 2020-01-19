use clap::{App, Arg, ArgMatches};

use crate::cli::*;
use crate::rpc::{VarlinkClient, VarlinkClientInterface};

pub(super) struct SetCommandSubcommand;

impl<'a> CliCommand<'a> for SetCommandSubcommand {
    fn get_name(&self) -> &'static str {
        "set-command"
    }

    #[rustfmt::skip]
    fn get_app(&self) -> App<'a> {
        self.get_base_app()
            .about("Set the docker command")
            .arg(Arg::with_name("command")
                // TODO: This for now must be an option, but we would prefer positional:
                // https://github.com/clap-rs/clap/issues/1437
                .long("command")
                .short('C')
                .help("The command for the container")
                .allow_hyphen_values(true)
                .multiple(true)
                .required_unless("unset"))
            .arg(Arg::with_name("unset")
                .help("Unset the command instead of setting it")
                .long_help(concat!(
                    "Unset the command instead of setting it. The container will use the ",
                    "default command."
                ))
                .long("unset")
                .short('u')
                .required_unless("command"))
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
            // Unset the command
            client
                .container_set_command(None, container.map(Into::into))
                .call()?;
        } else {
            // Set the command
            client
                .container_set_command(
                    Some(
                        args.value_of("command")
                            .expect("Missing required argument command")
                            .split(" ")
                            .map(ToOwned::to_owned)
                            .collect(),
                    ),
                    container.map(Into::into),
                )
                .call()?;
        }

        Ok(data)
    }
}
