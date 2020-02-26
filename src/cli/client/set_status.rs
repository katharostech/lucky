use clap::{App, Arg, ArgMatches, ArgSettings};

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
            .arg(Arg::with_name("status_name")
                .long("name")
                .short('n')
                .help("The name of the status to set: defaults to the script's name.")
                .long_help(
                    "The name of the status to set: defaults to the script's name. By default, \
                    setting the status will set the status only for the current script, \
                    leaving other statuses untouched. If you specify a name for the status \
                    other scripts can change that status by specifying the same name when \
                    calling `set-status`."
                )
                .env("LUCKY_SCRIPT_ID")
                .required(true))
            .arg(Arg::with_name("state")
                .help("The enumerated state of the service")
                .possible_values(&ScriptState::variants())
                .case_insensitive(true))
            .arg(Arg::with_name("message")
                .help("An optional message to provide with the state")
                .setting(ArgSettings::AllowEmptyValues)
                .required(false))
    }

    fn get_subcommands(&self) -> Vec<Box<dyn CliCommand<'a>>> {
        vec![]
    }

    fn get_doc(&self) -> Option<CliDoc> {
        Some(CliDoc {
            name: "lucky_client_set-status",
            content: include_str!("doc/set-status.md"),
        })
    }

    fn execute_command(&self, args: &ArgMatches, mut data: CliData) -> anyhow::Result<CliData> {
        let state = args
            .value_of("state")
            .expect("Missing required argument: state");
        let status = ScriptStatus {
            state: state.parse()?,
            message: args.value_of("message").map(ToOwned::to_owned),
        };
        let status_name = args
            .value_of("status_name")
            .expect("Missing required argument: status-name");

        // Get client connection
        let mut client: Box<VarlinkClient> = data
            .remove("client")
            .expect("Missing client data")
            .downcast()
            .expect("Invalid type");

        // Set script status
        client
            .set_status(status_name.into(), status.into())
            .call()?;

        Ok(data)
    }
}
