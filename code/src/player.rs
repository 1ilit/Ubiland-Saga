use glium::glutin::event::VirtualKeyCode;
use glium::{Display, Frame, Program};

use crate::input_mgr::InputManager;
use crate::shape::SCREEN_HEIGHT;
use crate::texture::{AnimatedTexture, Transform};

pub struct Player {
    pub texture: AnimatedTexture,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub velocity: [f32; 2],
}

impl Player {
    pub fn new(display: &Display) -> Self {
        let texture = AnimatedTexture::new(
            display,
            vec![
                "./res/ubi1.png",
                "./res/ubi2.png",
                "./res/ubi3.png",
                "./res/ubi4.png",
            ],
            0.2,
            3,
        );
        let (width, height) = texture.get_dimensions();
        Player {
            texture: texture,
            x: 0.0,
            y: 0.0,
            width: width,
            height: height,
            velocity: [0.0, 0.0],
        }
    }

    pub fn update(&mut self, input: &mut InputManager, dt: f32) {
        self.texture.update(dt);
        self.x += self.velocity[0];
        self.y += self.velocity[1];
        
        if self.y + self.velocity[1] - self.texture.height / 2.0 > -(SCREEN_HEIGHT / 2.) {
            self.velocity[1] -= 3.0 * dt;
        } else {
            self.velocity[1] = 0.0;
        }
        
        if input.key_down(VirtualKeyCode::Up) {
            self.velocity[1] = 380.0 * dt;
        }
        
        if input.key_down(VirtualKeyCode::Right) {
            self.velocity[0] = 200.0 * dt;
        } else if input.key_down(VirtualKeyCode::Left) {
            self.velocity[0] = -200.0 * dt;
        } else {
            self.velocity[0] = 0.0 * dt;
        }

        self.texture.set_position(self.x, self.y);
    }

    pub fn draw(&mut self, target: &mut Frame, program: &Program) {
        self.texture.draw(target, program);
    }
}
