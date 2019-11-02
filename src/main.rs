use std::fs;
use std::io;

const CHARM_TEMPLATE_ARCHIVE: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/charm_template.zip"));

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let zip_reader = std::io::Cursor::new(CHARM_TEMPLATE_ARCHIVE);
    let mut zip = zip::ZipArchive::new(zip_reader)?;

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

            if (&*file.name()).contains("hooks") {
                // Make hooks executable
                fs::set_permissions(&outpath, fs::Permissions::from_mode(0o744))?;
            } else if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode))?;
            }
        }
    }

    Ok(())
}
