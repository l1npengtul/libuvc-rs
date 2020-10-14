extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=uvc");

    let mut builder = bindgen::Builder::default();

    if cfg!(target_os = "freebsd"){
        builder = builder.clang_arg("-I/usr/local/include");
    }

    let bindings = builder.header("wrapper.h")
        .whitelist_function("uvc_.*")
        .whitelist_type("uvc_.*")
        .generate()
        .expect("Failed to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("uvc_bindings.rs"))
        .expect("Failed to write bindings");
}
