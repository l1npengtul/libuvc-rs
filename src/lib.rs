//! Safe wrapper around `libuvc`

extern crate uvc_sys;

mod context;
mod controls;
mod device;
mod error;
mod formats;
mod frame;
mod streaming;

use uvc_sys::*;

pub use streaming::{ActiveStream, StreamCtrl};

pub use context::Context;
pub use controls::{AutoExposureMode, AutoExposurePriority, ScanningMode};
pub use device::{
    DescriptionSubtype, Device, DeviceDescription, DeviceHandle, DeviceList, FormatDescriptor,
    FormatDescriptors, FrameDescriptor, FrameDescriptors,
};
pub use error::{Error, Result};
pub use formats::{Format, FrameFormat};
pub use frame::Frame;
