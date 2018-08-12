/*!
    Safe wrapper around `libuvc`

    This crate gives access to webcams connected to the computer,
    allowing one to stream and capture video.

    # How to use this crate

    ```no_run
    extern crate uvc;

    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;
    use std::time::Duration;

    fn main() {
        // Get a libuvc context
        let ctx = uvc::Context::new().expect("Could not get context");

        // Get a default device
        let dev = ctx
            .find_device(None, None, None)
            .expect("Could not find device");

        // Or create an iterator over all available devices
        let mut _devices = ctx.devices().expect("Could not enumerate devices");

        // The device must be opened to create a handle to the device
        let devh = dev.open().expect("Could not open device");

        // Most webcams support this format
        let format = uvc::StreamFormat {
            width: 640,
            height: 480,
            fps: 30,
            format: uvc::FrameFormat::YUYV,
        };

        // Get the necessary stream information
        let mut streamh = devh
            .get_stream_handle_with_format(format)
            .expect("Could not open a stream with this format");

        // This is a counter, increasing by one for every frame
        // This data must be 'static + Send + Sync to be used in
        // the callback used in the stream
        let counter = Arc::new(AtomicUsize::new(0));

        // Get a stream, calling the closure as callback for every frame
        let stream = streamh
            .start_stream(
                |_frame, count| {
                    count.fetch_add(1, Ordering::SeqCst);
                },
                counter.clone(),
            ).expect("Could not start stream");

        // Wait 10 seconds
        std::thread::sleep(Duration::new(10, 0));

        // Explicitly stop the stream
        // The stream would also be stopped
        // when going out of scope (dropped)
        stream.stop();
        println!("Counter: {}", counter.load(Ordering::SeqCst));
    }
    ```
    See also `mirror.rs` in the examples to get an example of how to capture and display a stream
  */
extern crate uvc_sys;

mod context;
mod controls;
mod device;
mod error;
mod formats;
mod frame;
mod streaming;

use uvc_sys::*;

pub use streaming::{ActiveStream, StreamHandle};

pub use context::Context;
pub use controls::{AutoExposureMode, AutoExposurePriority, ScanningMode};
pub use device::{
    DescriptionSubtype, Device, DeviceDescription, DeviceHandle, DeviceList, FormatDescriptor,
    FormatDescriptors, FrameDescriptor, FrameDescriptors,
};
pub use error::{Error, Result};
pub use formats::{FrameFormat, StreamFormat};
pub use frame::Frame;
