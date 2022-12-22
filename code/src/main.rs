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

    let texture = texture::Texture::new(
        "C:\\Users\\Lilit\\Desktop\\ubiland\\code\\res\\techno.png",
        &display,
    );

    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
        color: [f32; 4],
        tex_coords: [f32; 2],
    }

    implement_vertex!(Vertex, position, color, tex_coords);

    let tvertex1 = Vertex {
        position: [0.5, -0.5],
        color: [0.0, 0.0, 1.0, 1.0],
        tex_coords: [0.0, 0.0],
    };
    let tvertex2 = Vertex {
        position: [0.5, 0.5],
        color: [0.0, 1.0, 0.0, 1.0],
        tex_coords: [0.0, 0.0],
    };
    let tvertex3 = Vertex {
        position: [-0.5, -0.5],
        color: [1.0, 0.0, 1.0, 1.0],
        tex_coords: [1.0, 0.0],
    };

    let tshape = vec![tvertex1, tvertex2, tvertex3];

    let tvertex_buffer = glium::VertexBuffer::new(&display, &tshape).unwrap();

    let tindices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = include_str!("../shaders/vertex.glsl");

    let fragment_shader_src = include_str!("../shaders/fragment.glsl");

    let program =
        glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
            .unwrap();

    let mut t: f32 = -0.5;
    event_loop.run(move |ev, _, control_flow| {
        t += 0.0002;
        if t > 0.5 {
            t = -0.5;
        }

        let tuniforms = uniform! {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [ t-0.5 , 0.0, 0.0, 1.0],
            ],
            isTex: false,
        };

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);

        texture.draw(&mut target, &program);

        target
            .draw(
                &tvertex_buffer,
                &tindices,
                &program,
                &tuniforms,
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
