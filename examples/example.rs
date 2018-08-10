extern crate uvc_rs;

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;

fn main() {
    // Get a libuvc context
    let ctx = uvc_rs::Context::new().expect("Could not open context");

    // Get a default device
    let dev = ctx
        .find_device(None, None, None)
        .expect("Could not find a device");

    // Alternatively get a device by iterating over possible devices
    // Which is an iterator, allowing one to use next() to find a suitable device
    let mut _devices = ctx.devices().expect("Could not enumerate devices");

    let devh = dev.open().expect("Could not open device");

    let format = uvc_rs::StreamFormat {
        width: 640,
        height: 480,
        fps: 30,
        format: uvc_rs::FrameFormat::YUYV,
    };

    let mut streamh = devh
        .get_stream_handle_with_format(format)
        .expect("Could not open stream with this format");

    let counter = Arc::new(AtomicUsize::new(0));
    let stream = streamh
        .start_stream(
            |_frame, count| {
                count.fetch_add(1, Ordering::SeqCst);
            },
            counter.clone(),
        ).expect("Could not start stream");

    std::thread::sleep(Duration::new(3, 0));
    stream.stop();

    println!("Counter: {}", counter.load(Ordering::Acquire));
}
