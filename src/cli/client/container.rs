use clap::{App, ArgMatches};

use crate::cli::*;

mod apply;
mod image;

pub(super) struct ContainerSubcommand;

impl<'a> CliCommand<'a> for ContainerSubcommand {
    fn get_name(&self) -> &'static str {
        "container"
    }

    #[rustfmt::skip]
    fn get_app(&self) -> App<'a> {
        self.get_base_app()
            .about("Manipulate the charm's container(s)")
    }

    fn get_subcommands(&self) -> Vec<Box<dyn CliCommand<'a>>> {
        vec![
            Box::new(image::ImageSubcommand),
            Box::new(apply::ApplySubcommand),
        ]
    }

    fn get_doc(&self) -> Option<CliDoc> {
        None
    }

    fn execute_command(&self, _args: &ArgMatches, data: CliData) -> anyhow::Result<CliData> {
        Ok(data)
    }
}
