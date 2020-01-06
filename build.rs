use std::env;
use std::fs::{File, OpenOptions};
use std::io::{prelude::*, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

use regex::Regex;
use walkdir::WalkDir;

fn main() {
    generate_varlink_code();

    package_charm_template();
}

fn generate_varlink_code() {
    let cargo_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    // Generate Rust source from Varlink RPC interface definition
    varlink_generator::cargo_build_tosource("src/rpc/lucky.rpc.varlink", /* rustfmt */ true);

    // Open output file
    let mut output_file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(cargo_dir.join("src/rpc/lucky_rpc.rs"))
        .unwrap();

    // Replace `Display` impl for the varlink Error types with our own
    let error_impl_regex =
        Regex::new(r"(?msU)impl ::std::fmt::Display for ErrorKind \{.*^\}").unwrap();
    let mut file_contents = String::new();
    output_file.read_to_string(&mut file_contents).unwrap();
    let file_contents =
        error_impl_regex.replace_all(&file_contents, "include!(\"lucky_rpc_err_impl.rs\");");

    // Overwrite output file with new contents
    output_file.set_len(0).unwrap();
    output_file.seek(SeekFrom::Start(0)).unwrap();
    output_file.write_all(file_contents.as_bytes()).unwrap();
}

fn package_charm_template() {
    let cargo_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // Package charm template ZIP for inclusion into the binary
    let charm_template_dir = cargo_dir.join("charm_template");
    println!(
        "cargo:rerun-if-changed={}",
        charm_template_dir.to_str().unwrap()
    );
    let charm_template_zip_path = out_dir.join("charm_template.zip");
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
        println!("cargo:rerun-if-changed={}", path.to_str().unwrap());
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
