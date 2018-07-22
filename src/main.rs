extern crate png;
extern crate uvc_sys;

mod context;
mod device;
mod error;
mod streaming;

use streaming::*;

pub use context::Context;
pub use device::{Device, DeviceHandle};
pub use error::UvcError;

use uvc_sys::*;

use png::HasParameters;
use std::fs::File;
use std::io::BufWriter;

fn frame_to_png(frame: &uvc_frame, count: &mut u32) {
    let new_frame = unsafe { &mut *uvc_allocate_frame((frame.width * frame.height * 3) as usize) };
    unsafe {
        uvc_any2rgb(frame as *const uvc_frame as *mut uvc_frame, new_frame);
    }

    let bytes =
        unsafe { std::slice::from_raw_parts(new_frame.data as *const u8, new_frame.data_bytes) };

    let file = File::create(format!("cam{}.png", frame.sequence)).unwrap();
    let w = &mut BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, frame.width, frame.height);
    encoder.set(png::ColorType::RGB).set(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    writer.write_image_data(&bytes).unwrap();

    unsafe {
        uvc_free_frame(new_frame);
    }

    *count += 1;
}

fn main() {
    let ctx = Context::new().unwrap();
    let list = ctx.get_devices().expect("Could not get list");
    let dev = list.first().unwrap();
    let description = dev.description().unwrap();
    println!("{:?}", description);

    let devh = dev.open().unwrap();

    let mut ctrl = devh
        .get_stream_ctrl_with_size_and_fps(640, 480, 30)
        .unwrap();

    println!("{:?}", ctrl.ctrl);

    let calls = 0;
    let stream = ctrl.start_streaming(&devh, frame_to_png, calls).unwrap();

    std::thread::sleep(std::time::Duration::from_secs(1));

    std::mem::drop(stream);

    println!("Hello, world {:?}!", calls);
}
