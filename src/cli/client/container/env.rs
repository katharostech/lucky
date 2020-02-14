use clap::{App, Arg, ArgMatches};

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
            .about("Set a value")
            .arg(Arg::with_name("key")
                .help("The key to set in the store"))
            .arg(Arg::with_name("value")
                .help(r#"The value to set "key" to"#)
                .long_help(r#"The value to set "key" to. If not specified, key will be removed."#)
                .setting(clap::ArgSettings::AllowEmptyValues)
                .required(false))
            .arg(super::container_arg())
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

        let value = args.value_of("value");

        // Get client connection
        let mut client: Box<VarlinkClient> = data
            .remove("client")
            .expect("Missing client data")
            .downcast()
            .expect("Invalid type");

        // Set the environment value. If value was not provided the environment var will be deleted.
        client
            .container_env_set(key.into(), value.map(Into::into), container.map(Into::into))
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

        // Set script status
        client
            .container_env_set(key.into(), None, container.map(Into::into))
            .call()?;

        Ok(data)
    }
}
