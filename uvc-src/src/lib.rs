//! Dummy crate to build a vendored version of libuvc

// Ensure we are linking to libusb
extern crate libusb_sys;
// Ensure we are linking to libjpeg
#[cfg(feature = "jpeg")]
extern crate mozjpeg_sys;
