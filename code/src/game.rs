use glium::{Display, Frame, Program};

use crate::input_mgr::InputManager;
use crate::texture::Texture;

pub struct Game {
    texture: Texture,
    pub input: InputManager,
}

impl Game {
    pub fn new(display: &Display) -> Self {
        let mut tex = Texture::new("./res/rect.png", display);
        tex.clip(0.0, 0.0, 32.0, 32.0);
        let input = InputManager::new();

        Game {
            texture: tex,
            input: input,
        }
    }

    pub fn update(&mut self) {
        if self
            .input
            .key_went_up(glium::glutin::event::VirtualKeyCode::Up)
        {
            println!("up is pressed");
        }
    }

    pub fn draw(&mut self, target: &mut Frame, program: &Program) {
        self.texture.draw(target, program);
    }
}
