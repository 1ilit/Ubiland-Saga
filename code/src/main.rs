// #[macro_use]
extern crate glium;
extern crate image;

mod game;
mod texture;
mod input_mgr;

fn main() {
    println!("Hello, world!");
    use glium::glutin::dpi::LogicalSize;
    use glium::{glutin, Surface};

    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
        .with_inner_size(LogicalSize::new(720, 600))
        .with_title(format!("hi"));
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let vertex_shader_src = include_str!("../shaders/vertex.glsl");

    let fragment_shader_src = include_str!("../shaders/fragment.glsl");

    let program =
        glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
            .unwrap();

    let mut game = game::Game::new(&display);

    event_loop.run(move |ev, _, control_flow| {
        let next_frame_time =
            std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        match ev {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::KeyboardInput {
                    device_id: _,
                    input,
                    is_synthetic: _,
                } => {
                    game.input.update(input.state, input.virtual_keycode.unwrap_or(glutin::event::VirtualKeyCode::Tab));
                },
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                }
                _ => return,
            },
            _ => (),
        }
        //update game
        game.update();

        let mut target = display.draw();
        target.clear_color(0.2, 0.2, 0.2, 1.0);

        //draw game
        game.draw(&mut target, &program);

        target.finish().unwrap();
    });
}
