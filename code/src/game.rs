use std::vec;

use glium::{Display, Frame, Program};

use crate::{
    input_mgr::InputManager,
    player::Player,
    texture::{AnimatedTexture, Texture, Transform}, shape::Direction,
};

#[derive(Debug, PartialEq)]
pub enum Size {
    Small = 2,
    Medium = 3,
    Large = 5,
    XLarge = 7,
}

#[derive(Debug, PartialEq)]
pub enum Type {
    Plain,
    Enemy,
    Fish,
}

pub struct Platform {
    width: f32,
    height: f32,
    x: f32,
    y: f32,
    texture: Texture,
    enemies: Vec<AnimatedTexture>,
    enemy_speed: f32,
    pub fish: Vec<Texture>,
}

impl Platform {
    pub fn new(display: &Display, size: Size, platform_type: Type) -> Self {
        let texture: Texture;
        let width: f32;
        let mut enemies: Vec<AnimatedTexture> = vec![];
        let mut fish: Vec<Texture> = vec![];
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

        (|| {
            if size == Size::Small {
                return;
            }

            let n: usize = (width / 48.0) as usize;

            match platform_type {
                Type::Enemy => {
                    let mut e1 = AnimatedTexture::new(
                        display,
                        vec!["./res/enemy1.png", "./res/enemy2.png"],
                        0.3,
                        2,
                    );
                    e1.set_position(0.0, 24.0);
                    enemies.push(e1);
                }
                Type::Fish => {
                    for i in 0..=n / 2 {
                        let mut f = Texture::new("./res/fish.png", display);
                        f.set_position(i as f32 * 48.0, 36.0);
                        fish.push(f);
                    }
                    for i in (1..=n / 2).rev() {
                        let mut f = Texture::new("./res/fish.png", display);
                        f.set_position(i as f32 * -48.0, 36.0);
                        fish.push(f);
                    }
                }
                _ => {}
            }
        })();

        Self {
            width: width,
            height: 96.0,
            x: 0.0,
            y: 0.0,
            texture: texture,
            enemies: enemies,
            enemy_speed: 0.08,
            fish: fish,
        }
    }

    pub fn set_position(&mut self, x: f32, y: f32) {
        self.texture.set_position(x, y);
        self.x = self.texture.x;
        self.y = self.texture.y;
        for i in 0..self.fish.len() {
            let x0 = self.fish[i].x;
            self.fish[i].set_x(x0 + x);
            self.fish[i].set_y(self.y + 72.0)
        }
        for i in 0..self.enemies.len() {
            let x0 = self.enemies[i].x;
            self.enemies[i].set_x(x0 + x);
            self.enemies[i].set_y(self.y + 80.0)
        }
    }

    pub fn translate(&mut self, x: f32, y: f32) {
        self.texture.translate(x, y);
        self.x = self.texture.x;
        self.y = self.texture.y;
        for i in 0..self.fish.len() {
            let x0 = self.fish[i].x;
            self.fish[i].set_x(x0 + x);
        }
        for i in 0..self.enemies.len() {
            let x0 = self.enemies[i].x;
            self.enemies[i].set_x(x0 + x);
        }
    }

    pub fn update(&mut self, display: &Display, dt: f32) {
        for i in 0..self.enemies.len() {
            self.enemies[i].update(dt);
            
            if self.enemies[i].x + 32.0 >= self.x + self.width / 2.
                || self.enemies[i].x - 32.0 <= self.x - self.width / 2.
            {
                self.enemy_speed *= -1.0;
                self.enemies[i].mirror(display, Direction::Horizontal);
            }
            self.enemies[i].translate(self.enemy_speed, 0.0);
        }
    }

    pub fn draw(&mut self, target: &mut Frame, program: &Program) {
        self.texture.draw(target, program);
        for i in 0..self.fish.len() {
            self.fish[i].draw(target, program);
        }
        for i in 0..self.enemies.len() {
            self.enemies[i].draw(target, program);
        }
    }
}

pub struct Game {
    player: Player,
    platforms: Vec<Platform>,
}

impl Game {
    pub fn new(display: &Display) -> Self {
        let p = Player::new(display);
        let mut pl = Platform::new(display, Size::Large, Type::Enemy);
        pl.set_position(100.0, -100.0);
        // let mut pl2 = Platform::new(display, Size::Large, Type::Fish);
        // pl2.translate(LEFT + 96.0, -30.0);

        Game {
            player: p,
            platforms: vec![pl],
        }
    }

    pub fn update(&mut self, input: &mut InputManager, display: &Display, dt: f32) {
        self.player.update(input, dt);
        for i in 0..1 {
            self.platforms[i].update(display, dt);
        }

        for i in 0..1 {
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
            }

            if self.player.x >= self.platforms[i].x - self.platforms[i].width / 2.
                && self.player.x <= self.platforms[i].x + self.platforms[i].width / 2.
            {
                self.player.on_platform = true;
                break;
            } else {
                self.player.on_platform = false;
            }
        }

        if self.player.right {
            for i in 0..1 {
                self.platforms[i].translate(-60.0 * dt, 0.0);
            }
        }
    }

    pub fn draw(&mut self, target: &mut Frame, program: &Program) {
        for i in 0..1 {
            self.platforms[i].draw(target, program);
        }

        self.player.draw(target, program);
    }
}
