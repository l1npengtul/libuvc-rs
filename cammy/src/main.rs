extern crate png;
extern crate uvc_rs;

use uvc_rs::*;

use png::HasParameters;
use std::fs::File;
use std::io::BufWriter;

use std::sync::{Arc, Mutex};

fn frame_to_png(frame: &Frame, count: &mut Arc<Mutex<u32>>) {
    let new_frame = frame.to_rgb().unwrap();

    let bytes = new_frame.to_bytes();

    let count = {
        let mut count = Mutex::lock(&count).unwrap();
        let copy = *count;
        *count += 1;
        copy
    };

    let file = File::create(format!("cam{}.png", count)).unwrap();
    let w = &mut BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, frame.width(), frame.height());
    encoder.set(png::ColorType::RGB).set(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    writer.write_image_data(&bytes).unwrap();
}

fn main() {
    let ctx = Context::new().expect("Could not create context");
    let mut list = ctx.devices().expect("Could not get devices");

    let dev = list.next().expect("No device available");

    let description = dev.description().unwrap();
    println!("{:#?}", description);

    let devh = dev.open().expect("Could not open device");

    let mut ctrl = devh
        .get_stream_ctrl_with_size_and_fps(640, 480, 30)
        .unwrap();

    println!("{:#?}", ctrl);

    let calls = Arc::new(Mutex::new(0u32));
    let stream = ctrl
        .start_streaming(&devh, frame_to_png, calls.clone())
        .unwrap();

    std::thread::sleep(std::time::Duration::from_secs(1));

    stream.stop();
    println!("Number of captured frames: {}", calls.lock().unwrap());
}
