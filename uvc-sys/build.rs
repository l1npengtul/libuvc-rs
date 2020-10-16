extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    let mut includedir = None;
    if std::env::var_os("CARGO_FEATURE_VENDOR").is_some() {
        includedir = Some(std::env::var("DEP_UVCSRC_INCLUDE").unwrap());
    } else {
        println!("cargo:rustc-link-lib=uvc");
        if cfg!(target_os = "freebsd") {
            includedir = Some("/usr/local/include".to_owned());
        }
    }

    let mut builder = bindgen::Builder::default();

    if let Some(include) = includedir {
        builder = builder.clang_arg(format!("-I{}", include));
    }

    let bindings = builder
        .header("wrapper.h")
        .whitelist_function("uvc_.*")
        .whitelist_type("uvc_.*")
        .generate()
        .expect("Failed to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("uvc_bindings.rs"))
        .expect("Failed to write bindings");
}
