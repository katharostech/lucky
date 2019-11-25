use clap::{App, Arg, ArgMatches};

use std::collections::HashMap;

use crate::cli::daemon::get_daemon_client;
use crate::cli::doc;
use crate::daemon::rpc::VarlinkClientInterface;
use crate::types::{ScriptState, ScriptStatus};

#[rustfmt::skip]
/// Return the subcommand
pub(crate) fn get_subcommand<'a>() -> App<'a> {
    crate::cli::new_app("set-status")
        .arg(doc::get_arg())
        .about("Set the status of the current script")
        .arg(Arg::with_name("script_id")
            .long("script-id")
            .short('I')
            .help("The ID of the script that is being run")
            .long_help(concat!(
                "The ID of the script that is being run. Allows each script to have a status ",
                "independent of the other scripts in the charm."
            ))
            .env("LUCKY_SCRIPT_ID")
            .required(true))
        .arg(Arg::with_name("state")
            .required(true)
            .help("The enumerated state of the service")
            .possible_values(&ScriptState::variants())
            .case_insensitive(true))
        .arg(Arg::with_name("message")
            .help("An optional message to provide with the state")
            .required(false))
}

/// Run the subcommand
pub(crate) fn run(args: &ArgMatches, socket_path: &str) -> anyhow::Result<()> {
    // Show the docs if necessary
    doc::show_doc(
        &args,
        get_subcommand(),
        "lucky_clent_set-status",
        include_str!("set_status/set_status.md"),
    )?;

    let state = args
        .value_of("state")
        .expect("Missing required argument: state");
    let status = ScriptStatus {
        state: state.parse()?,
        message: args.value_of("message").map(|x| x.to_owned()),
    };
    let script_id = args
        .value_of("script_id")
        .expect("Missing required argument: script_id");

    // Connect to lucky daemon
    let mut client = get_daemon_client(socket_path)?;

    let mut environment = HashMap::<String, String>::new();
    for &var in &["JUJU_RELATION", "JUJU_RELATION_ID", "JUJU_REMOTE_UNIT", "JUJU_CONTEXT_ID"] {
        if let Ok(value) = std::env::var(var) {
            environment.insert(var.into(), value);
        }
    }

    // Set script status
    client.set_status(script_id.into(), status.into(), environment).call()?;

    Ok(())
}
