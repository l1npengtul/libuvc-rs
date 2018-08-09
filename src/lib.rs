//! Safe wrapper around `libuvc`

extern crate uvc_sys;

mod context;
mod device;
mod error;
mod frame;
mod streaming;

use uvc_sys::*;

pub use streaming::{ActiveStream, StreamCtrl};

pub use context::Context;
pub use device::{
    DescriptionSubtype, Device, DeviceDescription, DeviceHandle, DeviceList, Format,
    FormatDescriptor, FormatDescriptors, FrameDescriptor, FrameDescriptors,
};
pub use error::{Error, Result};
pub use frame::{Frame, FrameFormat};
