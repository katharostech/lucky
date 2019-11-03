use std::fs;
use std::io;

use clap::{App, Arg, ArgMatches, SubCommand};

pub(crate) fn get_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("create")
        .about("Create a new lucky charm.")
        .arg(Arg::with_name("target_dir")
            .required(true))
}

pub(crate) fn run(args: &ArgMatches) {
    println!("Creating charm");
    println!("{:?}", args);

    let zip_reader = std::io::Cursor::new(crate::CHARM_TEMPLATE_ARCHIVE);
    let mut zip = zip::ZipArchive::new(zip_reader).unwrap();

    for i in 0..zip.len() {
        let mut file = zip.by_index(i).unwrap();
        let outpath = file.sanitized_name();

        if (&*file.name()).ends_with('/') {
            fs::create_dir_all(&outpath).unwrap();
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p).unwrap();
                }
            }
            let mut outfile = fs::File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }

        // Get and Set permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
            }
        }
    }
}
