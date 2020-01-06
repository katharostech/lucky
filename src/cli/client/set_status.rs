use clap::{App, Arg, ArgMatches};

use crate::cli::*;
use crate::rpc::{VarlinkClient, VarlinkClientInterface};

use crate::types::{ScriptState, ScriptStatus};

pub(super) struct SetStatusSubcommand;

impl<'a> CliCommand<'a> for SetStatusSubcommand {
    fn get_name(&self) -> &'static str {
        "set-status"
    }

    #[rustfmt::skip]
    fn get_app(&self) -> App<'a> {
        self.get_base_app()
            .about("Set the status of the current script")
            .arg(Arg::with_name("script_id")
                .long("script-id")
                .short('I')
                .help("The ID of the script that is being run")
                .long_help(concat!(
                    "The ID of the script that is being run. Allows each script to have a status ",
                    "independent of the other scripts in the charm."
                ))
                .env("LUCKY_SCRIPT_ID")
                .required_unless("doc"))
            .arg(Arg::with_name("state")
                .required_unless("doc")
                .help("The enumerated state of the service")
                .possible_values(&ScriptState::variants())
                .case_insensitive(true))
            .arg(Arg::with_name("message")
                .help("An optional message to provide with the state")
                .required(false))
    }

    fn get_subcommands(&self) -> Vec<Box<dyn CliCommand<'a>>> {
        vec![]
    }

    fn get_doc(&self) -> Option<CliDoc> {
        None
    }

    fn execute_command(&self, args: &ArgMatches, mut data: CliData) -> anyhow::Result<CliData> {
        let state = args
            .value_of("state")
            .expect("Missing required argument: state");
        let status = ScriptStatus {
            state: state.parse()?,
            message: args.value_of("message").map(ToOwned::to_owned),
        };
        let script_id = args
            .value_of("script_id")
            .expect("Missing required argument: script_id");

        // Get client connection
        let mut client: Box<VarlinkClient> = data
            .remove("client")
            .expect("Missing client data")
            .downcast()
            .expect("Invalid type");

        // Set script status
        client.set_status(script_id.into(), status.into()).call()?;

        Ok(data)
    }
}
