use std::fs;
use std::io;
use std::path::PathBuf;

use clap::{App, Arg, ArgMatches, SubCommand};
use handlebars::Handlebars;
use serde::Serialize;

#[derive(Serialize, Default)]
struct TemplateData {
    pub charm_display_name: String,
    pub charm_name: String,
    pub charm_summary: String,
    pub charm_maintainer: String,
}

pub(crate) fn get_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("create")
        .about("Create a new lucky charm.")
        .arg(Arg::with_name("target_dir").required(true))
}

pub(crate) fn run(args: &ArgMatches) {
    // Create handlebars tempate engine
    let handlebars = Handlebars::new();

    // Initialize template
    let template_settings = TemplateData::default();

    // Create the zip reader from the embeded charm template archive
    let zip_reader = std::io::Cursor::new(crate::CHARM_TEMPLATE_ARCHIVE);
    let mut zip = zip::ZipArchive::new(zip_reader).unwrap();

    // Iterate through the items in the zip
    for i in 0..zip.len() {
        let mut file = zip.by_index(i).unwrap();
        let mut outpath = PathBuf::from(args.value_of("target_dir").unwrap());
        outpath.push(file.sanitized_name());

        // If file entry is a directory
        if file.name().ends_with('/') {
            // Create a directory
            fs::create_dir_all(&outpath).unwrap();

        // If it is a file
        } else {
            // If the file has a parent
            if let Some(p) = outpath.parent() {
                // If the parent doesn't exist yet
                if !p.exists() {
                    // Create the parent directories
                    fs::create_dir_all(&p).unwrap();
                }
            }

            // If the file is a handlebars template
            if file.name().ends_with(".hbs") {
                // Strip the `.hbs` extension from the output file path
                outpath = PathBuf::from(&outpath.to_str().unwrap().rsplitn(2, ".hbs").nth(1).unwrap());

                // Render the template to the output file
                let mut outfile = fs::File::create(&outpath).unwrap();
                handlebars
                    .render_template_source_to_write(&mut file, &template_settings, &mut outfile)
                    .unwrap();

            // If it is a normal file
            } else {
                // Create file and write contents
                let mut outfile = fs::File::create(&outpath).unwrap();
                io::copy(&mut file, &mut outfile).unwrap();
            }
        }

        // If we are on a unix system
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            // If there is a mode set for the file in the zip
            if let Some(mode) = file.unix_mode() {
                // Set ther permissions on the created file
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
            }
        }
    }
}
