fn main() {
    let jpeg_include = std::env::var_os("DEP_JPEG_INCLUDE").unwrap();
    let mut jpeg_paths = std::env::split_paths(&jpeg_include);
    let jpeg_include = jpeg_paths.next().unwrap();

    let jpeg_version = std::env::var("DEP_JPEG_LIB_VERSION").unwrap();
    let jpeg_lib_path = format!("{}/..", jpeg_include.to_str().unwrap(),);
    let jpeg_lib = format!("mozjpeg{}", jpeg_version);
    let jpeg_include2 = jpeg_paths.next().unwrap();

    let dst = cmake::Config::new("source")
        .define("ENABLE_UVC_DEBUGGING", "OFF")
        .define("CMAKE_BUILD_TARGET", "Static")
        .define("BUILD_EXAMPLE", "OFF")
        .define("JPEG_LIBRARY_RELEASE:PATH", &jpeg_lib_path)
        .define("JPEG_LIBRARY:PATH", &jpeg_lib_path)
        .define(
            "JPEG_INCLUDE_DIRS:PATH",
            format!(
                "{};{}",
                &jpeg_include.to_str().unwrap(),
                &jpeg_include2.to_str().unwrap()
            ),
        )
        .define(
            "JPEG_INCLUDE_DIR:PATH",
            format!(
                "{};{}",
                &jpeg_include.to_str().unwrap(),
                &jpeg_include2.to_str().unwrap()
            ),
        )
        .build();

    println!("cargo:rustc-link-lib=static={}", jpeg_lib);
    println!("cargo:rustc-link-search=native={}", jpeg_lib_path);
    println!("cargo:rustc-link-lib=static=uvc");
    println!("cargo:rustc-link-search=native={}/lib", dst.display());

    let bindings = bindgen::Builder::default()
        .clang_arg(format!("-I{}/include", dst.display()))
        .header(format!("{}/include/libuvc/libuvc.h", dst.display()))
        .whitelist_function("uvc_.*")
        .whitelist_type("uvc_.*")
        .generate()
        .expect("Failed to generate bindings");

    let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("uvc_bindings.rs"))
        .expect("Failed to write bindings");
}
