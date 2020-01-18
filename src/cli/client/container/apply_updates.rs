use clap::{App, ArgMatches};

use crate::cli::*;
use crate::rpc::{VarlinkClient, VarlinkClientInterface};

pub(super) struct ApplyUpdatesSubcommand;

impl<'a> CliCommand<'a> for ApplyUpdatesSubcommand {
    fn get_name(&self) -> &'static str {
        "apply-updates"
    }

    #[rustfmt::skip]
    fn get_app(&self) -> App<'a> {
        self.get_base_app()
            .unset_setting(clap::AppSettings::ArgRequiredElseHelp)
            .about("Apply pending container configuration updates")
    }

    fn get_subcommands(&self) -> Vec<Box<dyn CliCommand<'a>>> {
        vec![]
    }

    fn get_doc(&self) -> Option<CliDoc> {
        None
    }

    fn execute_command(&self, _args: &ArgMatches, mut data: CliData) -> anyhow::Result<CliData> {
        // Get client connection
        let mut client: Box<VarlinkClient> = data
            .remove("client")
            .expect("Missing client data")
            .downcast()
            .expect("Invalid type");

        // Apply the container configuration
        client.container_apply().call()?;

        Ok(data)
    }
}
