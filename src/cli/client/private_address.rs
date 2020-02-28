use clap::{App, ArgMatches};

use std::io::Write;

use crate::cli::*;
use crate::rpc::{VarlinkClient, VarlinkClientInterface};

pub(super) struct PrivateAddressSubcommand;

impl<'a> CliCommand<'a> for PrivateAddressSubcommand {
    fn get_name(&self) -> &'static str {
        "private-address"
    }

    #[rustfmt::skip]
    fn get_app(&self) -> App<'a> {
        self.get_base_app()
            .about("Get the private IP address of the unit")
            .unset_setting(AppSettings::ArgRequiredElseHelp)
    }

    fn get_subcommands(&self) -> Vec<Box<dyn CliCommand<'a>>> {
        vec![]
    }

    fn get_doc(&self) -> Option<CliDoc> {
        Some(CliDoc {
            name: "lucky_client_private-address",
            content: include_str!("cli_help/private_address.md"),
        })
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
            client.get_private_address().call()?.address
        )?;

        Ok(data)
    }
}
