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
            .setting(AppSettings::TrailingVarArg)
            .arg(Arg::with_name("unset")
                .help("Unset the command instead of setting it")
                .long_help(concat!(
                    "Unset the command instead of setting it. The container will use the ",
                    "default command."
                ))
                .long("unset")
                .short('u')
                .required_unless("command"))
            .arg(Arg::with_name("command")
                .help("The command for the container")
                .multiple(true)
                .required_unless("unset"))
            .arg(super::container_arg())
    }

    fn get_subcommands(&self) -> Vec<Box<dyn CliCommand<'a>>> {
        vec![]
    }

    fn get_doc(&self) -> Option<CliDoc> {
        Some(CliDoc {
            name: "lucky_client_container_set-command",
            content: include_str!("cli_help/set_command.md"),
        })
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
                        args.values_of("command")
                            .expect("Missing required argument: command")
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
