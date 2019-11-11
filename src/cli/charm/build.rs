use std::fs;
use std::path::Path;

use anyhow::Context;
use clap::{App, Arg, ArgMatches};
use walkdir::WalkDir;

use crate::cli::doc;
use crate::types::juju::{
    CharmMetadata, JUJU_NORMAL_HOOKS, JUJU_RELATION_HOOKS, JUJU_STORAGE_HOOKS,
};
use crate::types::lucky::LuckyMetadata;

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
            .help(concat!(
                "The directory to put the built charm in. Defaults to the `build` ",
                "directory in the charm_dir."))
            .long_help(concat!(
                "The directory to put the built charm in. Defaults to the `build` directory ",
                "in the charm dir. The built charm will be in `build_dir/charm_name`."))
            .long("build-dir")
            .short('b')
            .takes_value(true))
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
    let build_dir = if let Some(build_dir) = args.value_of("build_dir") {
        Path::new(build_dir).to_path_buf()
    } else {
        charm_path.join("build")
    };
    create_dir_all(&build_dir)?;

    // Load charm metadata
    let charm_metadata: CharmMetadata = load_yaml(&charm_path, "metadata")?;
    // Make sure there is a valid lucky.yaml file
    load_yaml::<LuckyMetadata>(&charm_path, "lucky")?;
    // Get charm name
    let charm_name = &charm_metadata.name;
    // Get build target dir
    let target_dir = build_dir.join(charm_name);

    // Clear the target directory
    if target_dir.exists() {
        fs::remove_dir_all(&target_dir).context(format!(
            "Could not remove build target directory: {:?}",
            target_dir
        ))?;
    }

    // Copy charm contents to build directory
    let build_dir_canonical = build_dir.canonicalize()?;
    for entry in WalkDir::new(charm_path).into_iter().filter_entry(|e| {
        // Don't include any files in the build dir
        let entry_path = if let Ok(path) = e.path().canonicalize() {
            path
        } else {
            // TODO: Handle this error with the not yet created `try_filter_entry`:
            // https://github.com/BurntSushi/walkdir/issues/131
            return false;
        };
        !(&entry_path == &build_dir_canonical)
    }) {
        let entry = entry?;
        let relative_path = entry
            .path()
            .strip_prefix(charm_path)
            .expect("Internal error parsing build paths");
        let source_path = entry.path();
        let target_path = target_dir.join(relative_path);

        // Create parent dir
        if let Some(parent) = &target_path.parent() {
            if !parent.exists() {
                create_dir_all(&parent)?;
            }
        }

        // Copy file
        if source_path.is_file() {
            fs::copy(source_path, &target_path).context(format!(
                "Could not copy file {:?} to {:?}",
                source_path, &target_path
            ))?;
        }
    }

    // Create bin dir
    let bin_dir = target_dir.join("bin");
    if !bin_dir.exists() {
        create_dir_all(&bin_dir)?;
    }

    // Create hook dir
    let hook_dir = target_dir.join("hooks");
    if !hook_dir.exists() {
        create_dir_all(&hook_dir)?;
    }

    // Copy in Lucky binary
    if !args.is_present("use_local_lucky") {
        // We will require the -l flag until our first release
        anyhow::bail!(concat!(
            "Currently the --use-local-lucky or -l flag is required to build a charm. Once we ",
            "have made our first release, lucky will be able to automatically download the ",
            "required version from GitHub so that it can run on whatever architecture the charm ",
            "is deployed to"
        ));
    } else {
        // Copy in the Lucky executable
        let lucky_path = bin_dir.join("lucky");
        let executable_path = std::env::current_exe()?;
        fs::copy(&executable_path, &lucky_path)?;

        // Create install hook
        let install_hook_path = hook_dir.join("install");
        write_file(&install_hook_path, include_str!("build/install-hook.sh"))?;
        set_file_mode(&install_hook_path, 0o755)?;
    }

    // Create normal Juju hooks ( those not specific to a relation or storage )
    for hook in JUJU_NORMAL_HOOKS {
        // Skip the install hook because we have already created it
        if hook == &"install" {
            continue;
        }
        let new_hook_path = hook_dir.join(hook);

        // Create hook from template
        write_file(
            &new_hook_path,
            &format!(include_str!("build/hook-template.sh"), hook_name = hook),
        )?;
        set_file_mode(&new_hook_path, 0o755)?;
    }

    // Create relation hooks
    let create_relation_hook = |relation_name: &String| -> anyhow::Result<()> {
        for hook_name_template in JUJU_RELATION_HOOKS {
            let hook_name = hook_name_template.replace("{}", relation_name);
            let new_hook_path = hook_dir.join(&hook_name);

            // Create hook from template
            write_file(
                &new_hook_path,
                &format!(
                    include_str!("build/hook-template.sh"),
                    hook_name = hook_name
                ),
            )?;
            set_file_mode(&new_hook_path, 0o755)?;
        }

        Ok(())
    };
    if let Some(relation_names) = charm_metadata.provides {
        relation_names.keys().try_for_each(create_relation_hook)?;
    }
    if let Some(relation_names) = charm_metadata.requires {
        relation_names.keys().try_for_each(create_relation_hook)?;
    }
    if let Some(relation_names) = charm_metadata.peers {
        relation_names.keys().try_for_each(create_relation_hook)?;
    }

    // Create hooks for defined storages
    if let Some(storage_data) = charm_metadata.storage {
        for storage_name in storage_data.keys() {
            for hook_name_template in JUJU_STORAGE_HOOKS {
                let hook_name = hook_name_template.replace("{}", storage_name);
                let new_hook_path = hook_dir.join(&hook_name);

                // Create hook from template
                write_file(
                    &new_hook_path,
                    &format!(
                        include_str!("build/hook-template.sh"),
                        hook_name = hook_name
                    ),
                )?;
                set_file_mode(&new_hook_path, 0o755)?;
            }
        }
    }

    Ok(())
}

//
// Utilities
//

/// Loads a yaml file with either a `.yml` or `.yaml` extension into the given type
fn load_yaml<T>(dir_path: &Path, base_name: &str) -> anyhow::Result<T>
where
    T: serde::de::DeserializeOwned,
{
    let file_path = if dir_path.join(format!("{}.yaml", base_name)).exists() {
        dir_path.join(format!("{}.yaml", base_name))
    } else {
        dir_path.join(format!("{}.yml", base_name))
    };
    if !file_path.exists() {
        anyhow::bail!(
            "Could not locate a {}.yaml file in the charm directory: {:?}",
            base_name,
            &dir_path
        );
    }
    let file_content =
        fs::read_to_string(&file_path).context(format!("Could not read file: {:?}", file_path))?;
    let data: T = serde_yaml::from_str(&file_content)
        .context(format!("Could not parse YAML: {:?}", file_path))?;

    Ok(data)
}

fn write_file(path: &Path, content: &str) -> anyhow::Result<()> {
    fs::write(&path, content)
        .context(format!("Could not write file: {:?}", &path))?;
    Ok(())
}

/// `fs::create_dir_all` with extra error context
fn create_dir_all(path: &Path) -> anyhow::Result<()> {
    fs::create_dir_all(&path).context(format!("Could not create dir: {:?}", path))?;
    Ok(())
}

/// Sets file permission mode on Unix with extra error context
fn set_file_mode(path: &Path, mode: u32) -> anyhow::Result<()> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(mode)).context(format!(
            "Could not set permissions on created file: {:?}",
            path
        ))?;
    }

    Ok(())
}
