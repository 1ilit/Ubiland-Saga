use glium::glutin::event::VirtualKeyCode;
use glium::{Display, Frame, Program};

use crate::input_mgr::InputManager;
use crate::shape::{SCREEN_HEIGHT};
use crate::texture::Texture;

pub struct Player {
    pub texture: Texture,
    pub position: [f32; 2],
    pub velocity: [f32; 2],
}

impl Player {
    pub fn new(display: &Display) -> Self {
        let texture = Texture::new("./res/star.png", display);
        Player {
            texture: texture,
            position: [0.0, 0.0],
            velocity: [0.0, 0.0],
        }
    }

    pub fn update(&mut self, input: &mut InputManager) {
        self.position[0] += self.velocity[0];
        self.position[1] += self.velocity[1];

        if self.position[1] + self.velocity[1] - self.texture.height / 2.0 > -(SCREEN_HEIGHT / 2.) {
            self.velocity[1] -= 0.4;
        } else {
            self.velocity[1] = 0.0;
        }

        if input.key_down(VirtualKeyCode::Up) {
            self.velocity[1] = 7.;
        }

        self.texture
            .set_position(self.position[0], self.position[1]);
    }

    pub fn draw(&mut self, target: &mut Frame, program: &Program) {
        self.texture.draw(target, program);
    }
}