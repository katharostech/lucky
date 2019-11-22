use anyhow::Context;
use clap::{App, ArgMatches};

mod set_status;

use crate::cli::daemon::{get_daemon_connection_args, get_daemon_socket_path};
use crate::cli::doc;

#[rustfmt::skip]
/// Return the `client` subcommand
pub(crate) fn get_subcommand<'a>() -> App<'a> {
    crate::cli::new_app("client")
        .about("Communicate with the Lucky daemon in charm scripts")
        .subcommand(set_status::get_subcommand())
        .arg(doc::get_arg())
        .args(&get_daemon_connection_args())
}

/// Run the `client` subcommand
pub(crate) fn run(args: &ArgMatches) -> anyhow::Result<()> {
    // Show the docs if necessary
    doc::show_doc(
        &args,
        get_subcommand(),
        "lucky_clent",
        include_str!("client/client.md"),
    )?;

    crate::log::init_default_logger()?;

    let socket_path = get_daemon_socket_path(&args);

    // Run a subcommand
    match args.subcommand() {
        ("set-status", Some(sub_args)) => {
            set_status::run(sub_args, &socket_path).context("Could not set status")
        }
        _ => get_subcommand()
            .write_help(&mut std::io::stderr())
            .map_err(|e| e.into()),
    }
}
