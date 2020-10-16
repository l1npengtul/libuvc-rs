struct Version {
    major: usize,
    minor: usize,
    patch: usize,
}

const VERSION: Version = Version {
    major: 0,
    minor: 0,
    patch: 6,
};

fn main() {
    let mut builder = cc::Build::new();
    builder.file("source/src/ctrl.c");
    builder.file("source/src/ctrl-gen.c");
    builder.file("source/src/device.c");
    builder.file("source/src/diag.c");
    builder.file("source/src/frame.c");
    builder.file("source/src/init.c");
    builder.file("source/src/stream.c");
    builder.file("source/src/misc.c");

    let builddir: std::path::PathBuf = std::env::var_os("OUT_DIR").unwrap().into();
    let includedir = builddir.join("include");
    {
        // Copy includedir
        std::fs::create_dir_all(includedir.join("libuvc")).unwrap();
        {
            let config_h = format!(
                r#"
#ifndef LIBUVC_CONFIG_H
#define LIBUVC_CONFIG_H
#define LIBUVC_VERSION_MAJOR {major}
#define LIBUVC_VERSION_MINOR {minor}
#define LIBUVC_VERSION_PATCH {patch}
#define LIBUVC_VERSION_STR "{major}.{minor}.{patch}"
#define LIBUVC_VERSION_INT (({major} << 16) | ({minor} << 8) | ({patch}))
#define LIBUVC_VERSION_GTE(major, minor, patch) (LIB_UVC_VERSION_INT >= (((major) << 16) | ((minor) << 8) | (patch))
#define LIBUVC_HAS_JPEG {has_jpeg}
#endif
"#,
                major = VERSION.major,
                minor = VERSION.minor,
                patch = VERSION.patch,
                has_jpeg = std::env::var_os("CARGO_FEATURE_JPEG").is_some() as u8,
            );
            use std::io::Write;
            let mut uvc_internal =
                std::fs::File::create(includedir.join("libuvc/libuvc_config.h")).unwrap();
            uvc_internal.write_all(config_h.as_bytes()).unwrap();
        }
        std::fs::copy(
            "source/include/libuvc/libuvc.h",
            includedir.join("libuvc/libuvc.h"),
        )
        .unwrap();
        std::fs::copy(
            "source/include/libuvc/libuvc_internal.h",
            includedir.join("libuvc/libuvc_internal.h"),
        )
        .unwrap();
        std::fs::copy(
            "source/include/utlist.h",
            includedir.join("libuvc/utlist.h"),
        )
        .unwrap();
        builder.include(&includedir);
    }

    if std::env::var_os("CARGO_FEATURE_JPEG").is_some() {
        builder.file("source/src/frame-mjpeg.c");
        let jpeg_includes = std::env::var_os("DEP_JPEG_INCLUDE").unwrap();
        for jpeg_include in std::env::split_paths(&jpeg_includes) {
            builder.include(jpeg_include);
        }
    }

    let usb_include = std::env::var_os("DEP_USB_1.0_INCLUDE").unwrap();
    builder.include(usb_include);

    builder.compile("uvc");

    println!("cargo:include={}", includedir.to_str().unwrap());
}
