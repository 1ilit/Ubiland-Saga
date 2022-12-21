#[macro_use]
extern crate glium;
extern crate image;

mod texture;

fn main() {
    println!("Hello, world!");
    use glium::{glutin, Surface};

    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new().with_title(format!("hi"));
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

// let image = image::load(Cursor::new(&include_bytes!("../res/techno.png")),
//                         image::ImageFormat::Png).unwrap().to_rgba8();
// let image_dimensions = image.dimensions();
// let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);

// let texture = glium::texture::SrgbTexture2d::new(&display, image).unwrap();

    let texture = texture::Texture::new("C:\\Users\\Lilit\\Desktop\\ubiland\\code\\res\\techno.png", &display);

    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
        color: [f32; 4],
        tex_coords: [f32; 2],
    }

    implement_vertex!(Vertex, position, color, tex_coords);

    let vertex1 = Vertex {
        position: [0.5, -0.5],
        color: [0.0, 0.0, 1.0, 1.0],
        tex_coords: [0.0, 0.0],
    };
    let vertex2 = Vertex {
        position: [0.5, 0.5],
        color: [0.0, 1.0, 0.0, 1.0],
        tex_coords: [0.0, 1.0]
    };
    let vertex3 = Vertex {
        position: [-0.5, -0.5],
        color: [1.0, 0.0, 1.0, 1.0],
        tex_coords: [1.0, 0.0]
    };
    let vertex4 = Vertex {
        position: [-0.5, 0.5],
        color: [0.0, 0.0, 1.0, 1.0],
        tex_coords: [1.0, 1.0]
    };
    let shape = vec![vertex1, vertex2, vertex3, vertex4];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();

    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);

    let vertex_shader_src = r#"
    #version 140

    in vec2 position;
    in vec4 color; 
    in vec2 tex_coords;

    out vec4 ourColor;
    out vec2 v_tex_coords;

    uniform mat4 matrix;

    void main() {
        v_tex_coords = tex_coords;
        ourColor=color;
        gl_Position = matrix * vec4(position, 0.0, 1.0);
    }
    "#;

    let fragment_shader_src = r#"
    #version 140

    in vec2 v_tex_coords;

    out vec4 fragColor;
    in vec4 ourColor;

    uniform sampler2D tex;

    void main() {
        fragColor = texture(tex, v_tex_coords);
    }
    "#;

    let program =
        glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
            .unwrap();

    let mut t: f32 = -0.5;
    event_loop.run(move |ev, _, control_flow| {
        t += 0.0002;
        if t > 0.5 {
            t = -0.5;
        }

        let uniforms = uniform! {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [ t , 0.0, 0.0, 1.0f32],
            ],
            tex: &texture.texture,
        };
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target
            .draw(
                &vertex_buffer,
                &indices,
                &program,
                &uniforms,
                &Default::default(),
            )
            .unwrap();
        target.finish().unwrap();

        let next_frame_time =
            std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
        match ev {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                }
                _ => return,
            },
            _ => (),
        }
    });
}
