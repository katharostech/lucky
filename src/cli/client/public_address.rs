use clap::{App, ArgMatches};

use std::io::Write;

use crate::cli::*;
use crate::rpc::{VarlinkClient, VarlinkClientInterface};

pub(super) struct PublicAddressSubcommand;

impl<'a> CliCommand<'a> for PublicAddressSubcommand {
    fn get_name(&self) -> &'static str {
        "public-address"
    }

    #[rustfmt::skip]
    fn get_app(&self) -> App<'a> {
        self.get_base_app()
            .about("Get the public address of the unit")
            .long_about("Get the public address of the unit, which may be a DNS name")
            .unset_setting(AppSettings::ArgRequiredElseHelp)
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

        // Set script status
        writeln!(
            std::io::stdout(),
            "{}",
            client.get_public_address().call()?.address
        )?;

        Ok(data)
    }
}
