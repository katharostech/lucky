use clap::{App, AppSettings};

// Subcommands
mod charm;

pub fn run() {
    let args = get_cli().get_matches();

    match args.subcommand() {
        ("charm", Some(sub_args)) => charm::run(sub_args),

        _ => panic!("Unimplemented subcommand or failure to show help.")
    }
}

fn get_cli() -> App<'static, 'static> {
    let mut app = App::new("Lucky")
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about("The Lucky charm framework for Juju.")
        .global_setting(AppSettings::ColoredHelp)
        .setting(AppSettings::SubcommandRequiredElseHelp);

    app = app.subcommand(charm::get_subcommand());

    app
}