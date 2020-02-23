use clap::{App, Arg, ArgMatches};

use std::collections::HashMap;
use std::io::Write;

use crate::cli::*;
use crate::rpc::{VarlinkClient, VarlinkClientInterface};

pub(super) struct EnvSubcommand;

impl<'a> CliCommand<'a> for EnvSubcommand {
    fn get_name(&self) -> &'static str {
        "env"
    }

    #[rustfmt::skip]
    fn get_app(&self) -> App<'a> {
        self.get_base_app()
            .about("Get and set container environment variables")
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
            .about("Get an environment variable")
            .long_about(concat!(
                "Get an environment variable from the container. ",
                "If you leave `key` unspecified, all environment variables will be printed out, ",
                "one per line, in the format `key=value`."
            ))
            .arg(Arg::with_name("key")
                .help("The environment variable to get"))
            .arg(super::container_arg())
    }

    fn get_subcommands(&self) -> Vec<Box<dyn CliCommand<'a>>> {
        vec![]
    }

    fn get_doc(&self) -> Option<CliDoc> {
        None
    }

    fn execute_command(&self, args: &ArgMatches, mut data: CliData) -> anyhow::Result<CliData> {
        let key = args.value_of("key");
        let container = args.value_of("container");

        // Get client connection
        let mut client: Box<VarlinkClient> = data
            .remove("client")
            .expect("Missing client data")
            .downcast()
            .expect("Invalid type");

        // If a specific key was given
        if let Some(key) = key {
            // Print out the requested value
            let response = client
                .container_env_get(key.into(), container.map(Into::into))
                .call()?;

            writeln!(
                std::io::stdout(),
                "{}",
                response.value.unwrap_or_else(|| "".into())
            )?;

        // If no key was given
        } else {
            // Return all of the key-value pairs
            for pair in client
                .container_env_get_all(container.map(Into::into))
                .call()?
                .pairs
            {
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
            .about("Set environment variables")
            .arg(Arg::with_name("vars")
                .help("The vars to set on the relation as `key=value` pairs separated by spaces")
                .long_help("The vars to set on the relation as `key=value` pairs separated by \
                            spaces. Setting values to a null string will remove the environment \
                            var.")
                .required(true)
                .multiple(true))
            .arg(super::container_arg())
    }

    fn get_subcommands(&self) -> Vec<Box<dyn CliCommand<'a>>> {
        vec![]
    }

    fn get_doc(&self) -> Option<CliDoc> {
        None
    }

    fn execute_command(&self, args: &ArgMatches, mut data: CliData) -> anyhow::Result<CliData> {
        let container = args.value_of("container");
        let raw_env_vars = args.values_of("vars").expect("Missing required arg: vars");

        // Parse key-value pairs
        let env_vars = util::parse_kv_pairs(raw_env_vars)?;

        // Get client connection
        let mut client: Box<VarlinkClient> = data
            .remove("client")
            .expect("Missing client data")
            .downcast()
            .expect("Invalid type");

        // Set the environment value. If value was not provided the environment var will be deleted.
        client
            .container_env_set(env_vars, container.map(Into::into))
            .call()?;

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
            .arg(Arg::with_name("container")
                .help("The name of the container to delete the environment variable for")
                .short('c')
                .long("container")
                .takes_value(true))
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
        let container = args.value_of("container");

        // Get client connection
        let mut client: Box<VarlinkClient> = data
            .remove("client")
            .expect("Missing client data")
            .downcast()
            .expect("Invalid type");

        let mut env_vars = HashMap::new();
        env_vars.insert(key.into(), None);

        // Set key to none ( therefore deleting it )
        client
            .container_env_set(env_vars, container.map(Into::into))
            .call()?;

        Ok(data)
    }
}
