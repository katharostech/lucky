use clap::{App, Arg, ArgMatches};

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

    // Connect to lucky daemon
    let mut client = get_daemon_client(socket_path)?;

    // Connect to service and trigger the hook
    log::info!(r#"Triggering hook "{}""#, &hook_name);
    for response in client.trigger_hook(hook_name.clone()).more()? {
        let response = response?;
        if let Some(output) = response.output {
            log::info!("output: {}", output);
        }
    }

    log::info!(r#"Done running hook "{}""#, &hook_name);

    Ok(())
}
