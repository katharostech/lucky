use clap::{App, ArgMatches};

mod build;
mod create;

use crate::cli::*;

pub(crate) struct CharmSubcommand;

impl<'a> CliCommand<'a> for CharmSubcommand {
    fn get_name(&self) -> &'static str {
        "charm"
    }

    fn get_command(&self) -> App<'a> {
        self.get_base_app().about("Build and create Lucky charms")
    }

    fn get_subcommands(&self) -> Vec<Box<dyn CliCommand<'a>>> {
        vec![
            Box::new(build::BuildSubcommand),
            Box::new(create::CreateSubcommand),
        ]
    }

    fn get_doc(&self) -> Option<CliDoc> {
        None
    }

    fn execute_command(&self, _args: &ArgMatches) -> anyhow::Result<()> {
        Ok(())
    }
}
