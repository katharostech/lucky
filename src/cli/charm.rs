use clap::{App, ArgMatches};

mod create;

use crate::cli::doc;

pub(crate) fn get_subcommand<'a>() -> App<'a> {
    crate::cli::new_app("charm")
        .about("Build and create Lucky charms.")
        .subcommand(create::get_subcommand())
        .subcommand(doc::get_subcommand())
}

pub(crate) fn run(args: &ArgMatches) -> anyhow::Result<()> {
    match args.subcommand() {
        ("create", Some(sub_args)) => create::run(sub_args),
        ("doc", _) => doc::run(include_str!("./charm/charm.md")),
        _ => panic!("Unimplemented subcommand or failure to show help."),
    }
}
