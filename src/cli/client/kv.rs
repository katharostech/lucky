use clap::{App, Arg, ArgMatches};

use std::collections::HashMap;
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
            .about("Set key-value data")
            .arg(Arg::with_name("data")
                .help("The data to set on the relation as `key=value` pairs separated by spaces")
                .required(true)
                .multiple(true))
    }

    fn get_subcommands(&self) -> Vec<Box<dyn CliCommand<'a>>> {
        vec![]
    }

    fn get_doc(&self) -> Option<CliDoc> {
        None
    }

    fn execute_command(&self, args: &ArgMatches, mut data: CliData) -> anyhow::Result<CliData> {
        let raw_kv_pairs = args.values_of("data").expect("Missing required arg: data");

        // Parse key-value pairs
        let kv_data = util::parse_kv_pairs(raw_kv_pairs)?;

        // Get client connection
        let mut client: Box<VarlinkClient> = data
            .remove("client")
            .expect("Missing client data")
            .downcast()
            .expect("Invalid type");

        // Set the key-value data
        client.unit_kv_set(kv_data).call()?;

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

        let mut kv_data = HashMap::new();
        kv_data.insert(key.into(), None);

        // Set key to None ( therefore deleting it )
        client.unit_kv_set(kv_data).call()?;

        Ok(data)
    }
}
