use std::env;
use std::fs::File;
use std::io::{prelude::*, Write};
use std::path::Path;

use walkdir::WalkDir;

fn main() {
    let charm_template_dir = format!("{}/charm_template", env::var("CARGO_MANIFEST_DIR").unwrap());
    let charm_template_zip_path = format!("{}/charm_template.zip", env::var("OUT_DIR").unwrap());
    let prefix = &charm_template_dir;

    let file_writer = File::create(&charm_template_zip_path).unwrap();
    let mut zip = zip::ZipWriter::new(file_writer);
    let options =
        zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Bzip2);

    let dir_iter = WalkDir::new(&charm_template_dir).into_iter();

    let mut buffer = Vec::new();
    for entry in dir_iter {
        let entry = entry.unwrap();
        let path = entry.path();
        let name = path.strip_prefix(Path::new(&prefix)).unwrap();

        if path.is_file() {
            zip.start_file_from_path(name, options).unwrap();
            let mut f = File::open(path).unwrap();

            f.read_to_end(&mut buffer).unwrap();
            zip.write_all(&*buffer).unwrap();
            buffer.clear();
        } else if name.as_os_str().len() != 0 {
            zip.add_directory_from_path(name, options).unwrap();
        }
    }
    zip.finish().unwrap();
}
