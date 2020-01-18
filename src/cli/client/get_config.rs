use clap::{App, Arg, ArgMatches};
use serde_json::Value as JsonValue;

use std::io::Write;

use crate::cli::*;
use crate::rpc::{VarlinkClient, VarlinkClientInterface};

pub(super) struct GetConfigSubcommand;

impl<'a> CliCommand<'a> for GetConfigSubcommand {
    fn get_name(&self) -> &'static str {
        "get-config"
    }

    #[rustfmt::skip]
    fn get_app(&self) -> App<'a> {
        self.get_base_app()
            .about("Get the charm configuration")
            .unset_setting(AppSettings::ArgRequiredElseHelp)
            .arg(Arg::with_name("key")
                .help("The config key to get")
                .long_help(concat!(
                    "The config key to get. If not specified all config values will be returned, ",
                    "one per line, in the format `key=value`."
                ))
                .takes_value(true))
    }

    fn get_subcommands(&self) -> Vec<Box<dyn CliCommand<'a>>> {
        vec![]
    }

    fn get_doc(&self) -> Option<CliDoc> {
        None
    }

    #[allow(clippy::filter_map)]
    fn execute_command(&self, args: &ArgMatches, mut data: CliData) -> anyhow::Result<CliData> {
        let key = args.value_of("key");

        // Get client connection
        let mut client: Box<VarlinkClient> = data
            .remove("client")
            .expect("Missing client data")
            .downcast()
            .expect("Invalid type");

        // Get config from daemon
        let config = client.get_config().call()?.config;

        // If the config key is specified
        if let Some(key) = key {
            // Get the specified config
            let value = config
                .iter()
                .filter(|x| x.key == key)
                .map(|x| serde_json::from_str(&x.value))
                .next()
                .unwrap_or(Ok(JsonValue::Null))?;

            // Print the value
            writeln!(std::io::stdout(), "{}", json_value_to_string(value))?;

        // If no key was specified
        } else {
            // For every config
            for pair in config {
                // Print the value
                writeln!(
                    std::io::stdout(),
                    "{}={}",
                    pair.key,
                    json_value_to_string(serde_json::from_str(&pair.value)?)
                )?;
            }
        }

        Ok(data)
    }
}

fn json_value_to_string(v: JsonValue) -> String {
    match v {
        JsonValue::Null => "".into(),
        JsonValue::String(s) => s.into(),
        other_json => other_json.to_string(),
    }
}
