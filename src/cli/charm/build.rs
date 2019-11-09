use clap::{App, ArgMatches};

use crate::cli::doc;

/// Return the `build` subcommand
pub(crate) fn get_subcommand<'a>() -> App<'a> {
    crate::cli::new_app("build")
        .about("Build a Lucky charm and make it ready for deployment")
        .arg(doc::get_arg())
}

/// Run the `build` subcommand
pub(crate) fn run(args: &ArgMatches) -> anyhow::Result<()> {
    // Show the docs if necessary
    doc::show_doc(
        &args,
        get_subcommand(),
        "lucky_charm_build",
        include_str!("build/build.md"),
    )?;

    Ok(())
}
