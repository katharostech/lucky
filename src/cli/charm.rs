use clap::{App, ArgMatches, SubCommand};

mod create;

use crate::cli::man;

pub(crate) fn get_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("charm")
        .about("Build and create Lucky charms.")
        .arg(man::arg())
        .subcommand(create::get_subcommand())
}

pub(crate) fn run(args: &ArgMatches) {
    man::manpage(&args, include_str!("charm/charm.md"));

    match args.subcommand() {
        ("create", Some(sub_args)) => create::run(sub_args),
        ("", None) => println!("TODO: show help"),
        _ => panic!("Unimplemented subcommand or failure to show help."),
    }
}
