use glium::{Display, Frame, Program};

use crate::{
    input_mgr::InputManager,
    player::Player,
    shape::{SCREEN_HEIGHT, SCREEN_WIDTH},
    texture::{Texture, Transform},
};

pub enum Size {
    Small,
    Medium,
    Large,
    XLarge,
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
            Size::XLarge => {
                texture = Texture::new("./res/platform_7.png", display);
                width = 336.0;
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

    pub fn translate(&mut self, x: f32, y: f32) {
        self.texture.translate(x, y);
        self.x += x;
        self.y += y;
    }

    pub fn draw(&mut self, target: &mut Frame, program: &Program) {
        self.texture.draw(target, program);
    }
}

pub struct Game {
    player: Player,
    platforms: Vec<Platform>,
}

impl Game {
    pub fn new(display: &Display) -> Self {
        let p = Player::new(display);
        let pl = Platform::new(display, Size::Large);
        let mut pl2 = Platform::new(display, Size::XLarge);
        pl2.translate(-SCREEN_WIDTH / 2.0 + 96.0, -30.0);

        Game {
            player: p,
            platforms: vec![pl, pl2],
        }
    }

    pub fn update(&mut self, input: &mut InputManager, dt: f32) {
        self.player.update(input, dt);

        for i in 0..2 {
            if self.player.x + self.player.width / 2.
                >= self.platforms[i].x - self.platforms[i].width / 2.
                && self.player.x - self.player.width / 2.
                    <= self.platforms[i].x + self.platforms[i].width / 2.
                && self.player.y - self.player.height / 2. + self.player.velocity[1]
                    <= self.platforms[i].y + self.platforms[i].height / 2.
                && self.player.y - self.player.height / 2.
                    >= self.platforms[i].y + self.platforms[i].height / 2.0
            {
                self.player.velocity[1] = 0.0;
                self.player.on_platform = true;
                break;
            } else {
                self.player.on_platform = false;
            }
        }

        if !self.player.on_platform {
            for i in 0..2 {
                self.platforms[i].translate(-0.01, 0.0);
            }
        }
    }

    pub fn draw(&mut self, target: &mut Frame, program: &Program) {
        for i in 0..2 {
            self.platforms[i].draw(target, program);
        }

        self.player.draw(target, program);
    }
}
