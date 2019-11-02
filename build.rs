use std::env;
use std::io::prelude::*;
use std::io::Write;
use zip::write::FileOptions;

use std::fs::File;
use std::path::Path;
use walkdir::WalkDir;

fn main() {
    // For now tar is required on the system compiling the project
    let manifest_dir_path = format!("{}/charm_template", env::var("CARGO_MANIFEST_DIR").unwrap());
    let charm_template_zip_path = format!("{}/charm_template.zip", env::var("OUT_DIR").unwrap());
    let prefix = env::var("CARGO_MANIFEST_DIR").unwrap();

    let file_writer = File::create(&charm_template_zip_path).unwrap();
    let mut zip = zip::ZipWriter::new(file_writer);
    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Bzip2)
        .unix_permissions(0o755);

    let dir_iter = WalkDir::new(&manifest_dir_path).into_iter();

    let mut buffer = Vec::new();
    for entry in dir_iter {
        let entry = entry.unwrap();
        let path = entry.path();
        eprintln!("{:?}", path);
        eprintln!("{:?}", prefix);
        let name = path.strip_prefix(Path::new(&prefix)).unwrap();

        // Write file or directory explicitly
        // Some unzip tools unzip files with directory paths correctly, some do not!
        if path.is_file() {
            println!("adding file {:?} as {:?} ...", path, name);
            zip.start_file_from_path(name, options).unwrap();
            let mut f = File::open(path).unwrap();

            f.read_to_end(&mut buffer).unwrap();
            zip.write_all(&*buffer).unwrap();
            buffer.clear();
        } else if name.as_os_str().len() != 0 {
            // Only if not root! Avoids path spec / warning
            // and mapname conversion failed error on unzip
            println!("adding dir {:?} as {:?} ...", path, name);
            zip.add_directory_from_path(name, options).unwrap();
        }
    }
    zip.finish().unwrap();
}
