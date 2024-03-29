extern crate glium;
extern crate image;

mod background;
mod collision;
mod enemy;
mod game;
mod gui;
mod input_mgr;
mod platform;
mod player;
mod screen_mgr;
mod screens;
mod shape;
mod texture;

use crate::screen_mgr::ScreenMgr;
use std::time::Instant;

fn main() {
    use glium::glutin::dpi::PhysicalSize;
    use glium::{glutin, Surface};

    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
        .with_inner_size(PhysicalSize::new(768, 576))
        .with_title(format!("Ubiland Saga"))
        .with_resizable(false);
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let vertex_shader_src = include_str!("../shaders/vertex.glsl");

    let fragment_shader_src = include_str!("../shaders/fragment.glsl");

    let program =
        glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
            .unwrap();

    let mut screen_mgr = ScreenMgr::new(&display);

    let mut previous_frame_time = Instant::now();

    event_loop.run(move |ev, _, control_flow| {
        let current_time = Instant::now();
        let elapsed_time = current_time.duration_since(previous_frame_time);
        let delta_time = elapsed_time.as_secs_f32();
        previous_frame_time = current_time;

        let next_frame_time =
            std::time::Instant::now() + std::time::Duration::from_nanos(delta_time as u64);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        match ev {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::KeyboardInput {
                    device_id: _,
                    input,
                    is_synthetic: _,
                } => {
                    screen_mgr.input.update(
                        input.state,
                        input
                            .virtual_keycode
                            .unwrap_or(glutin::event::VirtualKeyCode::Tab),
                    );
                }
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                }
                _ => return,
            },
            _ => (),
        }
        //update game
        screen_mgr.update(&display, delta_time);

        if screen_mgr.exited() {
            *control_flow = glutin::event_loop::ControlFlow::Exit;
            return;
        }

        let mut target = display.draw();
        target.clear_color(1.0, 1.0, 1.0, 1.0);
        //draw game
        screen_mgr.draw(&mut target, &program);
        target.finish().unwrap();
    });
}
