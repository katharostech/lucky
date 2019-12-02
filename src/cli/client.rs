use clap::{App, ArgMatches};

mod set_status;

use crate::cli::*;

pub(crate) struct ClientSubcommand;

impl<'a> CliCommand<'a> for ClientSubcommand {
    fn get_name(&self) -> &'static str {
        "client"
    }

    fn get_command(&self) -> App<'a> {
        self.get_base_app()
            .about("Communicate with the Lucky daemon in charm scripts")
    }

    fn get_subcommands(&self) -> Vec<Box<dyn CliCommand<'a>>> {
        vec![Box::new(set_status::SetStatusSubcommand)]
    }

    fn get_doc(&self) -> Option<CliDoc> {
        None
    }

    fn execute_command(&self, _args: &ArgMatches) -> anyhow::Result<()> {
        Ok(())
    }
}
