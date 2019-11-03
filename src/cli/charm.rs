use clap::{App, AppSettings, ArgMatches, SubCommand};

mod create;

pub(crate) fn get_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("charm")
        .about("Build and create Lucky charms.")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(create::get_subcommand())
}

pub(crate) fn run(args: &ArgMatches) {
    match args.subcommand() {
        ("create", Some(sub_args)) => create::run(sub_args),
        _ => panic!("Unimplemented subcommand or failure to show help.")
    }
}