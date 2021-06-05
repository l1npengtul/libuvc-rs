# Safe rust wrapper around [libuvc](https://int80k.com/libuvc/doc/)

[![crates.io](https://img.shields.io/crates/v/uvc.svg)](https://crates.io/crates/uvc)
[![license](https://img.shields.io/crates/l/uvc.svg)](https://github.com/mulimoen/libuvc-rs/blob/master/LICENSE)

## Maintainance

This project is no longer maintained. Please open an issue if you are interested in taking over maintainance of this project or publish new versions on `crates.io`.


## What does this library do?


This library gives access to the webcam, and allows one to capture the video stream. An example of how to use this library can be found in the examples directory.

An error such as `Access` might be due to the program not having read/write access to the usb device. You can grant access to all users by changing the device permissions, for example with

```
chmod 0666 /dev/bus/usb/{BUS}/{DEVICE}
```

where BUS and DEVICE can be found with `lsusb` or by running the `mirror` example.

## Documentation
Documentation can be created with `cargo doc`

## Dependencies
To use this crate, the `libuvc` native dependency must be installed, or vendored using the `vendor` feature. Disable the default-features and choose the feature `vendor` or `system` to select supplier.
