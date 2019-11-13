use clap::{App, Arg, ArgMatches};

use crate::cli::doc;

#[rustfmt::skip]
/// Return the `start` subcommand
pub(crate) fn get_subcommand<'a>() -> App<'a> {
    crate::cli::new_app("start")
        .unset_setting(clap::AppSettings::ArgRequiredElseHelp)
        .about("Start the Lucky daemon")
        .arg(doc::get_arg())
        .arg(Arg::with_name("unit_name")
            .long("unit-name")
            .short('u')
            .help("The name of the Juju unit that this daemon is running for")
            .takes_value(true)
            .env("JUJU_UNIT_NAME")
            .required_unless("socket_path"))
        .arg(Arg::with_name("socket_path")
            .long("socket-path")
            .short('s')
            .help("The path to the socket to listen on")
            .takes_value(true)
            .required_unless("unit_name"))
}

/// Run the `start` subcommand
pub(crate) fn run(args: &ArgMatches) -> anyhow::Result<()> {
    // Show the docs if necessary
    doc::show_doc(
        &args,
        get_subcommand(),
        "lucky_daemon_start",
        include_str!("start/start.md"),
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

    let service = crate::rpc::get_service();

    let listen_address = format!("unix:{};mode=700", socket_path);
    // TODO: Make this server shutdown gracefully. This might help:
    // https://docs.rs/varlink/8.1.0/varlink/enum.Listener.html
    varlink::listen(
        service,
        &listen_address,
        1,               // Min worker threads
        num_cpus::get(), // Max worker threads
        0,               // Timeout
    )?;

    Ok(())
}
