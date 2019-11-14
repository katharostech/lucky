// use anyhow::Context;
use clap::{App, Arg, ArgMatches};

// mod start;
// mod run_hook;

use crate::cli::doc;

#[rustfmt::skip]
/// Return the `client` subcommand
pub(crate) fn get_subcommand<'a>() -> App<'a> {
    crate::cli::new_app("client")
        .about("Communicate with the Lucky daemon in charm scripts")
        // .subcommand(start::get_subcommand())
        // .subcommand(run_hook::get_subcommand())
        .arg(doc::get_arg())
        // TODO: These are the exact same arguments as for the `daemon` subcommand:
        // we should probably remove the code duplication.
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

/// Run the `client` subcommand
pub(crate) fn run(args: &ArgMatches) -> anyhow::Result<()> {
    // Show the docs if necessary
    doc::show_doc(
        &args,
        get_subcommand(),
        "lucky_clent",
        include_str!("client/client.md"),
    )?;

    // Run a subcommand
    match args.subcommand() {
        // ("start", Some(sub_args)) => start::run(sub_args).context("Could not start daemon"),
        // ("run_hook", Some(sub_args)) => run_hook::run(sub_args).context("Could not run hook"),
        _ => panic!("Unimplemented subcommand or failure to show help."),
    }
}
