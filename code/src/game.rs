use glium::{Display, Frame, Program};

use crate::{input_mgr::InputManager, player::Player, texture::{Texture, Transform}};

pub enum Size {
    Small,
    Medium,
    Large,
}

pub struct Platform {
    width: u8,
    height: u8,
    position: [f32; 2],
    texture: Texture,
}

impl Platform {
    pub fn new(display: &Display, size: Size) -> Self {
        let texture: Texture;
        let width: u8;
        match size {
            Size::Small => {
                texture = Texture::new("./res/platform_2.png", display);
                width = 96;
            }
            Size::Medium => {
                texture = Texture::new("./res/platform_3.png", display);
                width = 144;
            }
            Size::Large => {
                texture = Texture::new("./res/platform_5.png", display);
                width = 240;
            }
        }

        Self {
            width: width,
            height: 96,
            position: [0.0, 0.0],
            texture: texture,
        }
    }

    pub fn draw(&mut self, target: &mut Frame, program: &Program) {
        self.texture.draw(target, program);
    }
}

pub struct Game {
    player: Player,
    platform: Platform,
}

impl Game {
    pub fn new(display: &Display) -> Self {
        let p = Player::new(display);
        let pl= Platform::new(display, Size::Small);

        Game { player: p , platform: pl,}
    }

    pub fn update(&mut self, input: &mut InputManager, dt: f32) {
        self.player.update(input, dt);
    }

    pub fn draw(&mut self, target: &mut Frame, program: &Program) {
        self.platform.draw(target, program);
        self.player.draw(target, program);
    }
}
