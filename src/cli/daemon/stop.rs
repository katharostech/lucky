use clap::{App, ArgMatches};

use crate::cli::doc;
use crate::daemon::{self, VarlinkClientInterface};

#[rustfmt::skip]
/// Return the `stop` subcommand
pub(crate) fn get_subcommand<'a>() -> App<'a> {
    crate::cli::new_app("stop")
        .unset_setting(clap::AppSettings::ArgRequiredElseHelp)
        .about("Stop the Lucky daemon")
        .arg(doc::get_arg())
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
    let connection = varlink::Connection::with_address(&connection_address)?;
    let mut service = daemon::get_client(connection);

    // Stop the daemon
    service.stop_daemon().call()?;

    println!( // TODO: logging
        "{} Shutdown server",
        crossterm::style::style("Success:").with(crossterm::style::Color::Green)
    );

    Ok(())
}
