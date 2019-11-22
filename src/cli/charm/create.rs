use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use anyhow::Context;
use clap::{App, Arg, ArgMatches};
use handlebars::Handlebars;
use rprompt::prompt_reply_stdout;
use serde::Serialize;

/// Zip archive data for the charm template
pub(crate) const CHARM_TEMPLATE_ARCHIVE: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/charm_template.zip"));

#[derive(Serialize)]
/// The input data to the charm template
struct TemplateData {
    pub charm_display_name: String,
    pub charm_name: String,
    pub charm_summary: String,
    pub charm_maintainer: String,
}

impl Default for TemplateData {
    fn default() -> Self {
        TemplateData {
            charm_display_name: String::from("My App"),
            charm_name: String::from("my_app"),
            charm_summary: String::from("A short summary of my app."),
            charm_maintainer: String::from("John Doe <johndoe@emailprovider.com>"),
        }
    }
}

use crate::cli::doc;

#[rustfmt::skip]
/// Return the `create` subcommand
pub(crate) fn get_subcommand<'a>() -> App<'a> {
    crate::cli::new_app("create")
        .about("Create a new lucky charm")
        .arg(doc::get_arg())
        .arg(Arg::with_name("target_dir")
            .help("The directory to create the charm in")
            .required_unless("doc"))
        .arg(Arg::with_name("use_defaults")
            .long("use-defaults")
            .short('D')
            .help("Do not prompt and use default values for unprovided fields"))
        .arg(Arg::with_name("charm_name")
            .long("name")
            .short('n')
            .help("The name of the charm. Defaults to the target_dir")
            .takes_value(true))
        .arg(Arg::with_name("display_name")
            .long("display-name")
            .short('d')
            .help("The display name of the charm ( may contain spaces )")
            .takes_value(true))
        .arg(Arg::with_name("charm_summary")
            .long("summary")
            .short('s')
            .help("Short description of the charm")
            .takes_value(true))
        .arg(Arg::with_name("charm_maintainer")
            .long("maintainer")
            .short('m')
            .help("The charm maintainer")
            .takes_value(true))       
}

/// Run the `create` subcommand
pub(crate) fn run(args: &ArgMatches) -> anyhow::Result<()> {
    doc::show_doc(
        args,
        get_subcommand(),
        "lucky_charm_create",
        include_str!("create/create.md"),
    )?;

    // Make sure target directory doesn't already exist
    let target_dir = Path::new(
        args.value_of("target_dir")
            .expect("Missing required argument: target_dir"),
    );
    if target_dir.exists() {
        anyhow::bail!("Error: target directory already exists");
    }

    // Create handlebars tempate engine
    let mut handlebars = Handlebars::new();
    // Clear the escape handler
    handlebars.register_escape_fn(handlebars::no_escape);

    // Initialize template settings
    let mut template_settings = TemplateData::default();

    // Set charm name
    if let Some(value) = args.value_of("charm_name") {
        template_settings.charm_name = String::from(value);
    }

    // Set display name
    if let Some(value) = args.value_of("display_name") {
        template_settings.charm_display_name = String::from(value);
    }

    // Set charm summary
    if let Some(value) = args.value_of("charm_summary") {
        template_settings.charm_summary = String::from(value);
    }

    // Set charm name
    if let Some(value) = args.value_of("charm_maintainer") {
        template_settings.charm_maintainer = String::from(value);
    }

    // If the defaults flag is not provided
    if !args.is_present("use_defaults") {
        // Prompt for missing display name
        if !args.is_present("display_name") {
            let default = target_dir
                .file_name()
                .map_or(target_dir.to_string_lossy(), |x| x.to_string_lossy());
            let response = prompt_reply_stdout(&format!("Display name [{}]: ", default))
                .context("Could not prompt for display name")?;
            let value = if response.trim() == "" {
                String::from(default)
            } else {
                response
            };
            template_settings.charm_display_name = value;
        }

        // Prompt for missing name
        if !args.is_present("charm_name") {
            let default = &template_settings
                .charm_display_name
                .replace(" ", "_")
                .to_lowercase();
            let response = prompt_reply_stdout(&format!("Charm name [{}]: ", default))
                .context("Could not prompt for charm name")?;
            let value = if response.trim() == "" {
                String::from(default)
            } else {
                response
            };
            template_settings.charm_name = value;
        }

        // Prompt for missing summary
        if !args.is_present("charm_summary") {
            let default = &template_settings.charm_summary;
            let response = prompt_reply_stdout(&format!("Charm summary [{}]: ", default))
                .context("Could not prompt for charm summary")?;
            let value = if response.trim() == "" {
                String::from(default)
            } else {
                response
            };
            template_settings.charm_summary = value;
        }

        // Prompt for missing maintainer
        if !args.is_present("charm_maintainer") {
            let default = &template_settings.charm_maintainer;
            let response = prompt_reply_stdout(&format!("Charm maintainer [{}]: ", default))
                .context("Could not prompt for charm maintainer")?;
            let value = if response.trim() == "" {
                String::from(default)
            } else {
                response
            };
            template_settings.charm_maintainer = value;
        }

    // User skipped prompts and opt-ed for default values
    } else {
        if !args.is_present("display_name") {
            template_settings.charm_display_name = target_dir
                .file_name()
                .map_or(target_dir.to_string_lossy(), |x| x.to_string_lossy())
                .to_string();
        }
        if !args.is_present("charm_name") {
            template_settings.charm_name = template_settings
                .charm_display_name
                .replace(" ", "_")
                .to_lowercase();
        }
    }

    // Create the zip reader from the embeded charm template archive
    let zip_reader = std::io::Cursor::new(CHARM_TEMPLATE_ARCHIVE);
    let zip_error_message = "Internal error: problem reading embedded charm template zip";
    let mut zip = zip::ZipArchive::new(zip_reader).context(zip_error_message)?;

    // Iterate through the items in the zip
    for i in 0..zip.len() {
        let mut file = zip.by_index(i).context(zip_error_message)?;
        let mut outpath = PathBuf::from(
            args.value_of("target_dir")
                .expect("missing required argument `target_dir`"),
        );
        outpath.push(file.sanitized_name());

        // If file entry is a directory
        if file.name().ends_with('/') {
            // Create a directory
            fs::create_dir_all(&outpath)
                .context(format!("Could not create directory: {:?}", outpath))?;

        // If it is a file
        } else {
            // If the file has a parent
            if let Some(p) = outpath.parent() {
                // If the parent doesn't exist yet
                if !p.exists() {
                    // Create the parent directories
                    fs::create_dir_all(&p)
                        .context(format!("Could not create directory: {:?}", p))?;
                }
            }

            // If the file is a handlebars template
            if file.name().ends_with(".hbs") {
                // Strip the `.hbs` extension from the output file path
                let panic_message = "Internal error parsing embedded template filename";
                outpath = PathBuf::from(
                    &outpath
                        .to_str()
                        .expect(panic_message)
                        .rsplitn(2, ".hbs")
                        .nth(1)
                        .expect(panic_message),
                );

                // Render the template to the output file
                let mut outfile = fs::File::create(&outpath).context(format!(
                    "Could not create file for charm template: {:?}",
                    outpath
                ))?;
                handlebars
                    .render_template_source_to_write(&mut file, &template_settings, &mut outfile)
                    .context(format!("Could not render template to file: {:?}", outfile))?;

            // If it is a normal file
            } else {
                // Create file and write contents
                let mut outfile = fs::File::create(&outpath)
                    .context(format!("Could not create file: {:?}", outpath))?;
                io::copy(&mut file, &mut outfile)
                    .context(format!("Could not write to file: {:?}", outpath))?;
            }
        }

        // If we are on a unix system
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            // If there is a mode set for the file in the zip
            if let Some(mode) = file.unix_mode() {
                // Set ther permissions on the created file
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).context(
                    format!("Could not set permissions on created file: {:?}", &outpath),
                )?;
            }
        }
    }

    Ok(())
}
