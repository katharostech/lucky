use anyhow::Context;
use clap::{App, ArgMatches};

mod build;
mod create;

use crate::cli::doc;

/// Return the `charm` subcommand
pub(crate) fn get_subcommand<'a>() -> App<'a> {
    crate::cli::new_app("charm")
        .about("Build and create Lucky charms")
        .subcommand(create::get_subcommand())
        .subcommand(build::get_subcommand())
        .arg(doc::get_arg())
}

/// Run the `charm` subcommand
pub(crate) fn run(args: &ArgMatches) -> anyhow::Result<()> {
    // Show the docs if necessary
    doc::show_doc(
        &args,
        get_subcommand(),
        "lucky_charm",
        include_str!("charm/charm.md"),
    )?;

    // Run a subcommand
    match args.subcommand() {
        ("create", Some(sub_args)) => create::run(sub_args).context("Could not create charm"),
        ("build", Some(sub_args)) => build::run(sub_args).context("Could not build charm"),
        _ => get_subcommand().write_help(&mut std::io::stderr()).map_err(|e| e.into()),
    }
}
