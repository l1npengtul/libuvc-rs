fn main() {
    println!("cargo:rustc-link-lib=usb-1.0");
    println!("cargo:rustc-link-lib=jpeg");

    let dst = cmake::Config::new("source")
        .define("ENABLE_UVC_DEBUGGING", "OFF")
        .define("CMAKE_BUILD_TARGET", "Static")
        .build();

    println!("cargo:rustc-link-lib=static=uvc");
    println!("cargo:rustc-link-search=native={}/lib", dst.display());

    let bindings = bindgen::Builder::default()
        .clang_arg(format!("-I{}/include", dst.display()))
        .header(format!("{}/include/libuvc/libuvc.h", dst.display()))
        .generate()
        .expect("Failed to generate bindings");

    let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("uvc_bindings.rs"))
        .expect("Failed to write bindings");
}
