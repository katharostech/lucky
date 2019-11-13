use clap::{App, ArgMatches};

use crate::cli::doc;
use crate::rpc::{self, VarlinkClientInterface};

/// Return the `run-hook` subcommand
pub(crate) fn get_subcommand<'a>() -> App<'a> {
    crate::cli::new_app("run-hook")
        .about("Run a hook through the Lucky daemon")
        .unset_setting(clap::AppSettings::ArgRequiredElseHelp)
        .arg(doc::get_arg())
}

/// Run the `run-hook` subcommand
pub(crate) fn run(args: &ArgMatches) -> anyhow::Result<()> {
    // Show the docs if necessary
    doc::show_doc(
        &args,
        get_subcommand(),
        "lucky_daemon_run-hook",
        include_str!("run_hook/run_hook.md"),
    )?;

    let connection = varlink::Connection::with_address("unix:/run/lucky.sock")?;
    let mut service = rpc::get_client(connection);
    service
        .run_hook("test-hook".to_string())
        .call()
        .map_err(|e| anyhow::anyhow!("Error running hook: {:?}", e))?;

    println!(
        "{} Ran hook!",
        crossterm::style::style("Success:").with(crossterm::style::Color::Green)
    );

    Ok(())
}
