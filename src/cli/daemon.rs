use anyhow::Context;
use clap::{App, Arg, ArgMatches};

mod start;
mod trigger_hook;

use crate::cli::doc;

#[rustfmt::skip]
/// Return the `daemon` subcommand
pub(crate) fn get_subcommand<'a>() -> App<'a> {
    crate::cli::new_app("daemon")
        .about("Run the Lucky daemon")
        .subcommand(start::get_subcommand())
        .subcommand(trigger_hook::get_subcommand())
        .arg(doc::get_arg())
        .arg(Arg::with_name("unit_name")
            .long("unit-name")
            .short('u')
            .help("The name of the Juju unit that this daemon is running for")
            .long_help(concat!(
                "The name of the Juju unit that this daemon is running for. This will be used to ",
                "determine path to the socket to listen on. For example a unit name of ",
                r#""mysql/2" would listen on the socket "/run/lucky_mysql_2.sock"."#
            ))
            .takes_value(true)
            .env("JUJU_UNIT_NAME")
            .required_unless("socket_path"))
        .arg(Arg::with_name("socket_path")
            .long("socket-path")
            .short('s')
            .help("The path to the socket to listen on")
            .long_help(concat!(
                "The path to the socket to listen on. This will override the path determined by ",
                "the unit-name argument."
            ))
            .takes_value(true)
            .required_unless("unit_name")
            .env("LUCKY_DAEMON_SOCKET"))
}

/// Run the `daemon` subcommand
pub(crate) fn run(args: &ArgMatches) -> anyhow::Result<()> {
    // Show the docs if necessary
    doc::show_doc(
        &args,
        get_subcommand(),
        "lucky_daemon",
        include_str!("daemon/daemon.md"),
    )?;

    let socket_path = match args.value_of("socket_path") {
        Some(path) => path.to_string(),
        None => format!(
            "/run/lucky_{}.sock",
            args.value_of("unit_name")
                .expect("Missing required argument: unit_name")
                .replace("/", "_")
        ),
    };

    // Run a subcommand
    match args.subcommand() {
        ("start", Some(sub_args)) => {
            start::run(sub_args, &socket_path).context("Could not start daemon")
        }
        ("trigger-hook", Some(sub_args)) => {
            trigger_hook::run(sub_args, &socket_path).context("Could not trigger hook")
        }
        _ => panic!("Unimplemented subcommand or failure to show help."),
    }
}
