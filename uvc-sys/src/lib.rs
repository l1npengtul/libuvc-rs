#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[cfg(feature = "vendor")]
extern crate uvc_src;

include!(concat!(env!("OUT_DIR"), "/uvc_bindings.rs"));
