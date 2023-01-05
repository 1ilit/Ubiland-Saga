use glium::{Display, Frame, Program, glutin::event::VirtualKeyCode};

use crate::{input_mgr::InputManager, player::Player, texture::{Texture, Transform}, shape::SCREEN_HEIGHT};

pub enum Size {
    Small,
    Medium,
    Large,
}

pub struct Platform {
    width: f32,
    height: f32,
    x: f32,
    y: f32,
    texture: Texture,
}

impl Platform {
    pub fn new(display: &Display, size: Size) -> Self {
        let texture: Texture;
        let width: f32;
        match size {
            Size::Small => {
                texture = Texture::new("./res/platform_2.png", display);
                width = 96.0;
            }
            Size::Medium => {
                texture = Texture::new("./res/platform_3.png", display);
                width = 144.0;
            }
            Size::Large => {
                texture = Texture::new("./res/platform_5.png", display);
                width = 240.0;
            }
        }

        Self {
            width: width,
            height: 96.0,
            x: 0.0,
            y: 0.0,
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
        let pl= Platform::new(display, Size::Large);

        Game { player: p , platform: pl,}
    }

    pub fn update(&mut self, input: &mut InputManager, dt: f32) {
        self.player.update(input, dt);

        if self.player.x+self.player.width/2.>=self.platform.x-self.platform.width/2. &&
           self.player.x-self.player.width/2.<=self.platform.x+self.platform.width/2. &&
           self.player.y-self.player.height/2.+self.player.velocity[1]<=self.platform.y+self.platform.height/2. &&
           self.player.y-self.player.height/2.>=self.platform.y+self.platform.height/2.0 {
            self.player.velocity[1] = 0.0;
        }
    }

    pub fn draw(&mut self, target: &mut Frame, program: &Program) {
        self.platform.draw(target, program);
        self.player.draw(target, program);
    }
}
