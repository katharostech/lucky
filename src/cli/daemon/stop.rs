use anyhow::Context;
use clap::{App, Arg, ArgMatches};

use crate::cli::doc;
use crate::daemon::{self, VarlinkClientInterface};

#[rustfmt::skip]
/// Return the `stop` subcommand
pub(crate) fn get_subcommand<'a>() -> App<'a> {
    crate::cli::new_app("stop")
        .unset_setting(clap::AppSettings::ArgRequiredElseHelp)
        .about("Stop the Lucky daemon")
        .arg(doc::get_arg())
        .arg(Arg::with_name("ignore_error")
            .long("ignore-error")
            .short('f')
            .help("Don't complain if we are unable to connect to the daemon"))
}

/// Run the `stop` subcommand
pub(crate) fn run(args: &ArgMatches, socket_path: &str) -> anyhow::Result<()> {
    // Show the docs if necessary
    doc::show_doc(
        &args,
        get_subcommand(),
        "lucky_daemon_stop",
        include_str!("stop/stop.md"),
    )?;

    // Connect to lucky daemon
    let connection_address = format!("unix:{}", &socket_path);
    let connection = varlink::Connection::with_address(&connection_address).or_else(|e| {
        if args.is_present("ignore_error") {
            std::process::exit(0);
        } else {
            Err(e).context(format!(
                r#"Could not connect to lucky daemon at: "{}""#,
                connection_address
            ))
        }
    })?;
    let mut service = daemon::get_client(connection);

    // Stop the daemon
    service.stop_daemon().call()?;

    log::info!(
        // TODO: logging
        "{} Shutdown server",
        crossterm::style::style("Success:").with(crossterm::style::Color::Green)
    );

    Ok(())
}
