use anyhow::format_err;
use clap::{App, Arg, ArgMatches};

use std::io::Write;

use crate::cli::*;
use crate::rpc::{RelationGet_Args_relation, VarlinkClient, VarlinkClientInterface};

pub(super) struct RelationSubcommand;

impl<'a> CliCommand<'a> for RelationSubcommand {
    fn get_name(&self) -> &'static str {
        "relation"
    }

    #[rustfmt::skip]
    fn get_app(&self) -> App<'a> {
        self.get_base_app()
            .setting(AppSettings::SubcommandRequiredElseHelp)
            .about("Communicate over Juju relations")
    }

    fn get_subcommands(&self) -> Vec<Box<dyn CliCommand<'a>>> {
        vec![
            Box::new(GetSubcommand),
            Box::new(SetSubcommand),
            Box::new(ListUnitsSubcommand),
            Box::new(ListIdsSubcommand),
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
            .about("Get data from a relation")
            .arg(Arg::with_name("relation_id")
                .help("The relation id to get the data from")
                .long("relation-id")
                .short('r')
                .takes_value(true)
                .requires("remote_unit_name"))
            .arg(Arg::with_name("remote_unit_name")
                .help("The name of the remote unit to get data from")
                .long("remote-unit")
                .short('u')
                .takes_value(true))
            .arg(Arg::with_name("app")
                .help("Get application relation data instead of unit relation data")
                .long("app")
                .short('A'))
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
        let relation_id = args.value_of("relation_id");

        // Get client connection
        let mut client: Box<VarlinkClient> = data
            .remove("client")
            .expect("Missing client data")
            .downcast()
            .expect("Invalid type");

        let app = args.is_present("app");

        let relation_data;
        if let Some(relation_id) = relation_id {
            let remote_unit_name = args.value_of("remote_unit_name").ok_or_else(|| {
                format_err!("--remote-unit option is required if specifying relation id")
            })?;

            relation_data = client
                .relation_get(
                    Some(RelationGet_Args_relation {
                        relation_id: relation_id.into(),
                        remote_unit: remote_unit_name.into(),
                    }),
                    app,
                )
                .call()?
                .data;
        } else {
            relation_data = client.relation_get(None, app).call()?.data;
        }

        // If a specific key was requested
        if let Some(key) = args.value_of("key") {
            writeln!(
                std::io::stdout(),
                "{}",
                relation_data.get(key).unwrap_or(&"".to_string()),
            )?;
        // Print all of the key-value pairs
        } else {
            for (k, v) in &relation_data {
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
            .about("Set local values on a relation")
            .arg(Arg::with_name("relation_id")
                .help("The relation id to get the data from")
                .long("relation-id")
                .short('r')
                .takes_value(true))
            .arg(Arg::with_name("app")
                .help("Set application relation data instead of unit relation data")
                .long_help(concat!(
                    "Set application relation data instead of unit relation data. the unit must ",
                    "be the leader unit to set application relation data",
                ))
                .long("app")
                .short('A'))
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
        // Get client connection
        let mut client: Box<VarlinkClient> = data
            .remove("client")
            .expect("Missing client data")
            .downcast()
            .expect("Invalid type");

        let relation_id = args.value_of("relation_id");
        let raw_kv_pairs = args.values_of("data").expect("Missing required arg: data");

        // Parse key-value pairs
        let relation_data = util::parse_kv_pairs(raw_kv_pairs)?;

        // Set relation data
        client
            .relation_set(
                relation_data,
                relation_id.map(Into::into),
                args.is_present("app"),
            )
            .call()?;

        Ok(data)
    }
}

struct ListUnitsSubcommand;

impl<'a> CliCommand<'a> for ListUnitsSubcommand {
    fn get_name(&self) -> &'static str {
        "list-units"
    }

    #[rustfmt::skip]
    fn get_app(&self) -> App<'a> {
        self.get_base_app()
            .about("List the units collected to a relation")
            .arg(Arg::with_name("relation_id")
                .help("The relation id to list the connected units for")
                .long("relation-id")
                .short('r')
                .takes_value(true))
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

        // Set script status
        let units = client
            .relation_list(args.value_of("relation_id").map(Into::into))
            .call()?
            .units;

        for unit in units {
            writeln!(std::io::stdout(), "{}", unit)?;
        }

        Ok(data)
    }
}

struct ListIdsSubcommand;

impl<'a> CliCommand<'a> for ListIdsSubcommand {
    fn get_name(&self) -> &'static str {
        "list-ids"
    }

    #[rustfmt::skip]
    fn get_app(&self) -> App<'a> {
        self.get_base_app()
            .about("List relation ids connected to this unit")
            .arg(Arg::with_name("relation_name")
                .help("The specific name of the relation ( from the metadata.yaml )")
                .long("relation-name")
                .short('n')
                .takes_value(true))
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

        // Set script status
        let ids = client
            .relation_ids(
                args.value_of("relation_name")
                    .expect("Missing required argument: relation-name")
                    .into(),
            )
            .call()?
            .ids;

        for id in ids {
            writeln!(std::io::stdout(), "{}", id)?;
        }

        Ok(data)
    }
}
