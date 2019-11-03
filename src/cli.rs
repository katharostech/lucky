use clap::{App, AppSettings};

pub fn run() {
    get_cli().get_matches();
}

fn get_cli() -> App<'static, 'static> {
    App::new("Lucky")
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about("The Lucky charm framework for Juju.")
        .global_setting(AppSettings::ColoredHelp)
        .setting(AppSettings::SubcommandRequiredElseHelp)
}