use glium::{implement_vertex, uniform};

use std::error::Error;

use glium::Surface;
use uvc::{Context, Frame};

fn frame_to_raw_image(
    frame: &Frame,
) -> Result<glium::texture::RawImage2d<'static, u8>, Box<dyn Error>> {
    let new_frame = frame.to_rgb()?;
    let data = new_frame.to_bytes();

    let image = glium::texture::RawImage2d::from_raw_rgb(
        data.to_vec(),
        (new_frame.width(), new_frame.height()),
    );

    Ok(image)
}

fn main() {
    let ctx = Context::new().expect("Could not create context");
    let dev = ctx
        .find_device(None, None, None)
        .expect("Could not find device");

    let description = dev.description().unwrap();
    println!(
        "Found device: Bus {:03} Device {:03} : ID {:04x}:{:04x} {} ({})",
        dev.bus_number(),
        dev.device_address(),
        description.vendor_id,
        description.product_id,
        description.product.unwrap_or_else(|| "Unknown".to_owned()),
        description
            .manufacturer
            .unwrap_or_else(|| "Unknown".to_owned())
    );

    // Open multiple devices by enumerating:
    // let mut list = ctx.devices().expect("Could not get devices");
    // let dev = list.next().expect("No device available");

    let devh = dev.open().expect("Could not open device");

    let format = devh
        .get_preferred_format(|x, y| {
            if x.fps >= y.fps && x.width * x.height >= y.width * y.height {
                x
            } else {
                y
            }
        })
        .unwrap();

    println!("Best format found: {:?}", format);
    let mut streamh = devh.get_stream_handle_with_format(format).unwrap();

    println!(
            "Scanning mode: {:?}\nAuto-exposure mode: {:?}\nAuto-exposure priority: {:?}\nAbsolute exposure: {:?}\nRelative exposure: {:?}\nAboslute focus: {:?}\nRelative focus: {:?}",
            devh.scanning_mode(),
            devh.ae_mode(),
            devh.ae_priority(),
            devh.exposure_abs(),
            devh.exposure_rel(),
            devh.focus_abs(),
            devh.focus_rel(),
        );

    use glium::glutin;
    let events_loop = glutin::event_loop::EventLoopBuilder::with_user_event().build();

    let evloop_proxy = events_loop.create_proxy();
    let _stream = streamh
        .start_stream(move |frame| {
            let image = frame_to_raw_image(frame);
            match image {
                Err(x) => println!("{:#?}", x),
                Ok(x) => {
                    let _ = evloop_proxy.send_event(x);
                }
            }
        })
        .unwrap();

    let window = glutin::window::WindowBuilder::new().with_title("Mirror");
    let context = glutin::ContextBuilder::new();
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
    )
    .unwrap();
    let program =
        glium::Program::from_source(&display, vertex_shader_source, fragment_shader_source, None)
            .unwrap();

    let mut buffer: Option<glium::texture::SrgbTexture2d> = None;
    events_loop.run(move |event, _, control_flow| {
        match event {
            glutin::event::Event::WindowEvent {
                event: glutin::event::WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = glutin::event_loop::ControlFlow::Exit;
                return;
            }
            glutin::event::Event::UserEvent(image) => {
                let image = glium::texture::SrgbTexture2d::new(&display, image)
                    .expect("Could not use image");
                buffer = Some(image);
            }
            _ => {}
        }

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);

        if let Some(ref b) = buffer {
            let uniforms = uniform! { u_image: b };
            target
                .draw(
                    &vertices,
                    &indices,
                    &program,
                    &uniforms,
                    &Default::default(),
                )
                .unwrap();
        }

        target.finish().unwrap();
    });
}
