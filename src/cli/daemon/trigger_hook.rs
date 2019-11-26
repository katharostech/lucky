use clap::{App, Arg, ArgMatches};

use std::collections::HashMap;

use crate::cli::daemon::get_daemon_client;
use crate::cli::doc;
use crate::daemon::rpc::VarlinkClientInterface;

#[rustfmt::skip]
/// Return the `trigger-hook` subcommand
pub(crate) fn get_subcommand<'a>() -> App<'a> {
    crate::cli::new_app("trigger-hook")
        .about("Run a hook through the Lucky daemon")
        .unset_setting(clap::AppSettings::ArgRequiredElseHelp)
        .arg(doc::get_arg())
        .arg(Arg::with_name("get_logs")
            .long("get-logs")
            .short('L')
            .help("Print the logs of the hook as it is running")
            .long_help(concat!(
                "Print the logs of the hook as it is running. Even if the logs are not printed ",
                "here, the standard out and error of the hook will be logged to Juju and can be ",
                "viewed with `juju debug-log`."
            )))
        .arg(Arg::with_name("hook_name")
            .help("The name of the hook to trigger")
            .required(true))
}

/// Run the `trigger-hook` subcommand
pub(crate) fn run(args: &ArgMatches, socket_path: &str) -> anyhow::Result<()> {
    // Show the docs if necessary
    doc::show_doc(
        &args,
        get_subcommand(),
        "lucky_daemon_trigger-hook",
        include_str!("trigger_hook/trigger_hook.md"),
    )?;

    let hook_name = args
        .value_of("hook_name")
        .expect("Missing required argument: hook_name")
        .to_string();

    // Populate environment variables the Lucky daemon may need for executing the hook
    let mut environment: HashMap<String, String> = HashMap::new();
    for &var in &[
        "JUJU_RELATION",
        "JUJU_RELATION_ID",
        "JUJU_REMOTE_UNIT",
        "JUJU_CONTEXT_ID",
    ] {
        if let Ok(value) = std::env::var(var) {
            environment.insert(var.into(), value);
        }
    }

    // Connect to lucky daemon
    let mut client = get_daemon_client(socket_path)?;

    log::info!(r#"Triggering hook "{}""#, &hook_name);

    // If the caller wants the hook logs
    if args.is_present("get_logs") {
        // Trigger the hook and stream the logs
        for response in client.trigger_hook(hook_name.clone(), environment).more()? {
            let response = response?;
            if let Some(output) = response.output {
                log::info!("output: {}", output);
            }
        }

    // If we don't care about the logs
    } else {
        // Just trigger the hook and exit
        client.trigger_hook(hook_name.clone(), environment).call()?;
    }

    log::info!(r#"Done running hook "{}""#, &hook_name);

    Ok(())
}
