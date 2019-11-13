use clap::{App, Arg, ArgMatches};

use crate::cli::doc;
use crate::rpc::{self, VarlinkClientInterface};

/// Return the `trigger-hook` subcommand
pub(crate) fn get_subcommand<'a>() -> App<'a> {
    crate::cli::new_app("trigger-hook")
        .about("Run a hook through the Lucky daemon")
        .unset_setting(clap::AppSettings::ArgRequiredElseHelp)
        .arg(doc::get_arg())
        .arg(
            Arg::with_name("hook_name")
                .help("The name of the hook to trigger")
                .required(true),
        )
}

/// Run the `trigger-hook` subcommand
pub(crate) fn run(args: &ArgMatches) -> anyhow::Result<()> {
    // Show the docs if necessary
    doc::show_doc(
        &args,
        get_subcommand(),
        "lucky_daemon_trigger-hook",
        include_str!("trigger_hook/trigger_hook.md"),
    )?;

    let connection = varlink::Connection::with_address("unix:/run/lucky.sock")?;
    let mut service = rpc::get_client(connection);
    service
        .trigger_hook(
            args.value_of("hook_name")
                .expect("Missing required argument: hook_name")
                .to_string(),
        )
        .call()
        .map_err(|e| anyhow::anyhow!("Error running hook: {:?}", e))?;

    println!(
        "{} Ran hook!",
        crossterm::style::style("Success:").with(crossterm::style::Color::Green)
    );

    Ok(())
}
