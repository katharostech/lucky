use std::path::Path;

use anyhow::Context;
use clap::{App, Arg, ArgMatches};

use crate::cli::doc;
use crate::types::CharmMetadata;

#[rustfmt::skip]
/// Return the `build` subcommand
pub(crate) fn get_subcommand<'a>() -> App<'a> {
    crate::cli::new_app("build")
        .about("Build a Lucky charm and make it ready for deployment")
        .long_about(concat!(
            "Build a Lucky charm and make it ready for deployment to the Juju ",
            "server or charm store"))
        .arg(doc::get_arg())
        .help_heading("LUCKY_INSTALL_SOURCE")
        .arg(Arg::with_name("use_local_lucky")
            .help("Build the charm with the local copy of lucky included")
            .long_help(include_str!("build/arg_use-local-lucky.txt"))
            .long("use-local-lucky")
            .short('l'))
        .stop_custom_headings()
        .arg(Arg::with_name("build_dir")
            .help("The directory to put the built charm in")
            .long_help(concat!(
                "The directory to put the built charm in. The built charm will be in ",
                "`build_dir/charm_name`."))
            .long("build-dir")
            .short('b')
            .default_value("build"))
        .arg(Arg::with_name("charm_dir")
            .help("The path to the charm you want to build")
            .required(false)
            .default_value("."))
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

    // Get charm dir
    let charm_path = Path::new(
        args.value_of("charm_dir")
            .expect("Missing required argument: charm_dir"),
    );

    // Create build dir
    let build_dir = Path::new(
        args.value_of("build_dir")
            .expect("Missing required argument: build_dir"),
    );
    std::fs::create_dir_all(&build_dir).context("Could not create build directory")?;

    // Load charm metadata
    let metadata_path = if charm_path.join("metadata.yaml").exists() {
        charm_path.join("metadata.yaml")
    } else {
        charm_path.join("metadata.yml")
    };
    if !metadata_path.exists() {
        anyhow::bail!(
            "Could not locate a metadata.yaml file in the given charm directory: {:?}",
            &charm_path
        );
    }
    let metadata_content = std::fs::read_to_string(&metadata_path)
        .context(format!("Couldn't read file: {:?}", metadata_path))?;
    let metadata: CharmMetadata =
        serde_yaml::from_str(&metadata_content).context("Couldn't parse charm metadata YAML")?;
    let charm_name = &metadata.name;

    let target_dir = build_dir.join(charm_name);

    Ok(())
}
