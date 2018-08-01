#[macro_use]
extern crate glium;
extern crate png;
extern crate uvc_rs;

use std::fs::File;
use std::io::BufWriter;
use std::sync::{Arc, Mutex};

use glium::Surface;
use png::HasParameters;
use uvc_rs::*;

#[allow(unused)]
fn callback_frame_to_png(frame: &Frame, count: &mut Arc<Mutex<u32>>) {
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

#[allow(unused)]
fn callback_frame_to_vec(frame: &Frame, count: &mut Arc<Mutex<Option<Frame>>>) {
    let new_frame = frame.to_rgb().unwrap();

    let mut data = Mutex::lock(&count).unwrap();

    *data = Some(new_frame);
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

    let calls = Arc::new(Mutex::new(None));
    let _stream = ctrl
        .start_streaming(&devh, callback_frame_to_vec, calls.clone())
        .unwrap();

    use glium::glutin;
    let mut events_loop = glium::glutin::EventsLoop::new();
    let window = glium::glutin::WindowBuilder::new();
    let context = glium::glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    #[derive(Copy, Clone)]
    pub struct QuadVertex {
        pos: (f32, f32),
    }

    implement_vertex!(QuadVertex, pos);

    let vertices: [QuadVertex; 4] = [
        QuadVertex { pos: (-1.0, -1.0) },
        QuadVertex { pos: (-1.0, 1.0) },
        QuadVertex { pos: (1.0, -1.0) },
        QuadVertex { pos: (1.0, 1.0) },
    ];

    let indices: [u8; 6] = [0, 1, 2, 1, 3, 2];

    let vertex_shader_source = r#"
    #version 140

    in vec2 pos;

    out vec2 v_position;

    void main() {
        v_position = (pos + 1.0)/2.0;
        gl_Position = vec4(-pos.x, -pos.y, 0.0, 1.0);
    }
    "#;

    let fragment_shader_source = r#"
    #version 140

    in vec2 v_position;

    out vec4 colour;

    uniform sampler2D u_image;

    void main() {
        vec2 pos = v_position;

        colour = texture(u_image, pos);
    }
    "#;

    let vertices = glium::VertexBuffer::new(&display, &vertices).unwrap();
    let indices = glium::IndexBuffer::new(
        &display,
        glium::index::PrimitiveType::TrianglesList,
        &indices,
    ).unwrap();
    let program =
        glium::Program::from_source(&display, vertex_shader_source, fragment_shader_source, None)
            .unwrap();

    let mut closed = false;
    let mut buffer = None;
    while !closed {
        events_loop.poll_events(|ev| match ev {
            glutin::Event::WindowEvent { event, .. } => match event {
                glutin::WindowEvent::CloseRequested => closed = true,
                _ => {}
            },
            _ => {}
        });

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);

        let mut mutex = Mutex::lock(&calls).unwrap();

        let frame: Option<Frame> = mutex.take();
        match frame {
            None => {
                // No new frame to render
            }
            Some(frame) => {
                let data_bytes = frame.to_bytes();

                let image = glium::texture::RawImage2d::from_raw_rgb(
                    data_bytes.to_vec(),
                    (frame.width(), frame.height()),
                );

                buffer = Some(
                    glium::texture::SrgbTexture2d::with_format(
                        &display,
                        image,
                        glium::texture::SrgbFormat::U8U8U8,
                        glium::texture::MipmapsOption::NoMipmap,
                    ).unwrap(),
                );
            }
        }

        if let Some(ref b) = buffer {
            let uniforms = uniform! { u_image: b };
            target
                .draw(
                    &vertices,
                    &indices,
                    &program,
                    &uniforms,
                    &Default::default(),
                ).unwrap();
        }

        target.finish().unwrap();
    }
}
