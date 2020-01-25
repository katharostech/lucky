use clap::{App, Arg, ArgMatches};

use std::io::Write;

use crate::cli::*;
use crate::rpc::{VarlinkClient, VarlinkClientInterface};

pub(super) struct KvSubcommand;

impl<'a> CliCommand<'a> for KvSubcommand {
    fn get_name(&self) -> &'static str {
        "kv"
    }

    #[rustfmt::skip]
    fn get_app(&self) -> App<'a> {
        self.get_base_app()
            .setting(AppSettings::SubcommandRequiredElseHelp)
            .about("Get and set values in the unit key-value store")
    }

    fn get_subcommands(&self) -> Vec<Box<dyn CliCommand<'a>>> {
        vec![
            Box::new(GetSubcommand),
            Box::new(SetSubcommand),
            Box::new(DeleteSubcommand),
        ]
    }

    fn get_doc(&self) -> Option<CliDoc> {
        None
    }

    fn execute_command(&self, _args: &ArgMatches, data: CliData) -> anyhow::Result<CliData> {
        Ok(data)
    }
}

struct GetSubcommand;

impl<'a> CliCommand<'a> for GetSubcommand {
    fn get_name(&self) -> &'static str {
        "get"
    }

    #[rustfmt::skip]
    fn get_app(&self) -> App<'a> {
        self.get_base_app()
            .unset_setting(clap::AppSettings::ArgRequiredElseHelp)
            .about("Get a value")
            .long_about(concat!(
                "Get a value from the key-value store. ",
                "If you leave `key` unspecified, all key-value pairs will be printed out, one ",
                "per line, in the format `key=value`."
            ))
            .arg(Arg::with_name("key")
                .help("The key to get from the store"))
    }

    fn get_subcommands(&self) -> Vec<Box<dyn CliCommand<'a>>> {
        vec![]
    }

    fn get_doc(&self) -> Option<CliDoc> {
        None
    }

    fn execute_command(&self, args: &ArgMatches, mut data: CliData) -> anyhow::Result<CliData> {
        let key = args.value_of("key");

        // Get client connection
        let mut client: Box<VarlinkClient> = data
            .remove("client")
            .expect("Missing client data")
            .downcast()
            .expect("Invalid type");

        // If a specific key was given
        if let Some(key) = key {
            // Print out the requested value
            let response = client.unit_kv_get(key.into()).call()?;

            writeln!(
                std::io::stdout(),
                "{}",
                response.value.unwrap_or_else(|| "".into())
            )?;

        // If no key was given
        } else {
            // Return all of the key-value pairs
            for pair in client.unit_kv_get_all().call()?.pairs {
                // Print out key-value pair
                writeln!(std::io::stdout(), "{}={}", pair.key, pair.value)?;
            }
        }

        Ok(data)
    }
}

struct SetSubcommand;

impl<'a> CliCommand<'a> for SetSubcommand {
    fn get_name(&self) -> &'static str {
        "set"
    }

    #[rustfmt::skip]
    fn get_app(&self) -> App<'a> {
        self.get_base_app()
            .about("Set a value")
            .arg(Arg::with_name("key")
                .help("The key to set in the store")
                .required(true))
            .arg(Arg::with_name("value")
                .help(r#"The value to set "key" to"#)
                .required(true))
    }

    fn get_subcommands(&self) -> Vec<Box<dyn CliCommand<'a>>> {
        vec![]
    }

    fn get_doc(&self) -> Option<CliDoc> {
        None
    }

    fn execute_command(&self, args: &ArgMatches, mut data: CliData) -> anyhow::Result<CliData> {
        let key = args
            .value_of("key")
            .expect("Missing required argument: key");
        let value = args
            .value_of("value")
            .expect("Missing required argument: value");

        // Get client connection
        let mut client: Box<VarlinkClient> = data
            .remove("client")
            .expect("Missing client data")
            .downcast()
            .expect("Invalid type");

        // Set script status
        client.unit_kv_set(key.into(), Some(value.into())).call()?;

        Ok(data)
    }
}

struct DeleteSubcommand;

impl<'a> CliCommand<'a> for DeleteSubcommand {
    fn get_name(&self) -> &'static str {
        "delete"
    }

    #[rustfmt::skip]
    fn get_app(&self) -> App<'a> {
        self.get_base_app()
            .about("Delete a value")
            .arg(Arg::with_name("key")
                .help("The key to delete from the store"))
    }

    fn get_subcommands(&self) -> Vec<Box<dyn CliCommand<'a>>> {
        vec![]
    }

    fn get_doc(&self) -> Option<CliDoc> {
        None
    }

    fn execute_command(&self, args: &ArgMatches, mut data: CliData) -> anyhow::Result<CliData> {
        let key = args
            .value_of("key")
            .expect("Missing required argument: key");

        // Get client connection
        let mut client: Box<VarlinkClient> = data
            .remove("client")
            .expect("Missing client data")
            .downcast()
            .expect("Invalid type");

        // Set script status
        client.unit_kv_set(key.into(), None).call()?;

        Ok(data)
    }
}
