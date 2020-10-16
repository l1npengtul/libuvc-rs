#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// Ensure we are linking to libusb
extern crate libusb_sys;
// Ensure we are linking to libjpeg
#[cfg(feature = "jpeg")]
extern crate mozjpeg_sys;

include!(concat!(env!("OUT_DIR"), "/uvc_bindings.rs"));
