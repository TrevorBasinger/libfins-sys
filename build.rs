use bindgen;
use cc;
use std::path::PathBuf;
use std::{env, fs};

fn lookup_src_files(src_dir: &str) -> Vec<String> {
    let entries = fs::read_dir(src_dir).unwrap();
    let mut src_files: Vec<String> = vec![];

    for entry in entries {
        let path = entry.unwrap().path();
        if !path.is_dir() {
            if let Some(pstr) = path.to_str() {
                src_files.push(pstr.to_string());
            }
        }
    }
    src_files
}

fn main() {
    let src_dir = "libfins/src";
    let header_dir = "libfins/include";

    let src_files = lookup_src_files(src_dir);

    if src_files.len() == 0 {
        panic!("Couldn't find source files. Make sure that you init submodules.");
    }

    println!("cargo:rustc-link-lib=fins");
    println!("cargo:rerun-if-changed={}/{}", header_dir, "fins.h");
    for f in src_files.iter() {
        println!("cargo:rerun-if-changed={}", f);
    }

    let mut builder = cc::Build::new();
    let build = builder
        .files(src_files)
        .include(header_dir)
        .compiler("gcc")
        .flag("-std=gnu99");
    build.compile("fins");

    println!("cargo:rerun-if-changed=src/wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("src/wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings.");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings.");
}
