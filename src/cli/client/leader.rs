use anyhow::format_err;
use clap::{App, Arg, ArgMatches};
use regex::Regex;

use std::collections::HashMap;
use std::io::Write;

use crate::cli::*;
use crate::rpc::{VarlinkClient, VarlinkClientInterface};

pub(super) struct LeaderSubcommand;

impl<'a> CliCommand<'a> for LeaderSubcommand {
    fn get_name(&self) -> &'static str {
        "leader"
    }

    #[rustfmt::skip]
    fn get_app(&self) -> App<'a> {
        self.get_base_app()
            .setting(AppSettings::SubcommandRequiredElseHelp)
            .about("Communicate as/with unit leader")
    }

    fn get_subcommands(&self) -> Vec<Box<dyn CliCommand<'a>>> {
        vec![
            Box::new(GetSubcommand),
            Box::new(SetSubcommand),
            Box::new(IsLeaderSubcommand),
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
            .about("Get data from the leader unit")
            .arg(Arg::with_name("key")
                .help("Optional key to get from the data")
                .required(false))
    }

    fn get_subcommands(&self) -> Vec<Box<dyn CliCommand<'a>>> {
        vec![]
    }

    fn get_doc(&self) -> Option<CliDoc> {
        None
    }

    fn execute_command(&self, args: &ArgMatches, mut data: CliData) -> anyhow::Result<CliData> {
        // Get client connection
        let mut client: Box<VarlinkClient> = data
            .remove("client")
            .expect("Missing client data")
            .downcast()
            .expect("Invalid type");

        let leader_data = client.leader_get().call()?.data;

        // If a specific key was requested
        if let Some(key) = args.value_of("key") {
            writeln!(
                std::io::stdout(),
                "{}",
                leader_data.get(key).unwrap_or(&"".to_string()),
            )?;
        // Print all key-value pairs
        } else {
            for (k, v) in &leader_data {
                writeln!(std::io::stdout(), "{}={}", k, v)?;
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
            .about("Set values as the leader charm")
            .arg(Arg::with_name("data")
                .help("The data to set as the leader as `key=value` pairs separated by spaces")
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
        // Get client connection
        let mut client: Box<VarlinkClient> = data
            .remove("client")
            .expect("Missing client data")
            .downcast()
            .expect("Invalid type");

        let raw_kv_pairs = args.values_of("data").expect("Missing required arg: data");

        // Create regex for matching key-value pairs
        let re = Regex::new(r"(?ms)^(?P<key>[a-zA-Z][a-zA-Z0-9]*)=(?P<value>.*)")
            .expect("Could not compile regex");

        // Parse key-value pairs
        let mut leader_data = HashMap::new();
        for raw_kv_pair in raw_kv_pairs {
            if let Some(captures) = re.captures(raw_kv_pair) {
                let key = captures.name("key").expect("Expected key").as_str();

                let value = captures.name("value").expect("Expected value").as_str();

                leader_data.insert(key.to_string(), value.to_string());
            } else {
                return Err(format_err!(
                    "Could not parse key-value pair: {}",
                    raw_kv_pair
                ));
            }
        }

        // Set leader data
        client.leader_set(leader_data).call()?;

        Ok(data)
    }
}

struct IsLeaderSubcommand;

impl<'a> CliCommand<'a> for IsLeaderSubcommand {
    fn get_name(&self) -> &'static str {
        "is-leader"
    }

    #[rustfmt::skip]
    fn get_app(&self) -> App<'a> {
        self.get_base_app()
            .unset_setting(clap::AppSettings::ArgRequiredElseHelp)
            .about("Get whether or not this unit is the leader unit")
            .long_about(concat!(
                "Get whether or not this unit is the leader unit. Returns \"true\" if unit is ",
                "leader and \"false\" if it is not."
            ))
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

        if client.leader_is_leader().call()?.is_leader {
            writeln!(std::io::stdout(), "true")?;
        } else {
            writeln!(std::io::stdout(), "false")?;
        }

        Ok(data)
    }
}