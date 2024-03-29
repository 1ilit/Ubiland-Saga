use std::vec;

use glium::{Display, Frame, Program};

use crate::{
    collision::{intersect, overlap_x},
    enemy::{Enemy, Species},
    gui::Topbar,
    player::Player,
    shape::Direction,
    texture::{Rect, Texture, Transform},
};

#[derive(Debug, PartialEq)]
pub enum Size {
    Small,
    Medium,
    Large,
    XLarge,
}

impl Size {
    pub fn from_u32(value: u32) -> Size {
        match value {
            0 => Size::Small,
            1 => Size::Medium,
            2 => Size::Large,
            _ => panic!("Unknown value: {}", value),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Type {
    Enemy,
    Fish,
    Plain,
}

pub struct Fish {
    pub texture: Texture,
    x: f32,
    y: f32,
    pub taken: bool,
}

impl Fish {
    pub fn new(display: &Display) -> Self {
        Self {
            texture: Texture::new("./res/platforms/fish.png", display),
            x: 0.0,
            y: 0.0,
            taken: false,
        }
    }

    pub fn set_position(&mut self, x: f32, y: f32) {
        self.texture.set_position(x, y);
        self.x = x;
        self.y = y;
    }

    pub fn translate(&mut self, x: f32, y: f32) {
        self.texture.translate(x, y);
        self.x = self.texture.x;
        self.y = self.texture.y;
    }

    pub fn set_x(&mut self, x: f32) {
        self.texture.set_x(x);
        self.x = x;
    }

    pub fn draw(&mut self, target: &mut Frame, program: &Program) {
        if !self.taken {
            self.texture.draw(target, program);
        }
    }
}

pub struct Platform {
    pub width: f32,
    pub height: f32,
    pub x: f32,
    pub y: f32,
    pub size: Size,
    pub texture: Texture,
    pub enemies: Vec<Enemy>,
    pub enemy_speed: f32,
    pub fish: Vec<Fish>,
    pub elapsed_time: f32,
    pub platform_type: Type,
}

impl Platform {
    pub fn new(display: &Display, size: Size) -> Self {
        let texture: Texture;
        let width: f32;
        match size {
            Size::Small => {
                texture = Texture::new("./res/platforms/small.png", display);
                width = 96.0;
            }
            Size::Medium => {
                texture = Texture::new("./res/platforms/medium.png", display);
                width = 144.0;
            }
            Size::Large => {
                texture = Texture::new("./res/platforms/large.png", display);
                width = 240.0;
            }
            Size::XLarge => {
                texture = Texture::new("./res/platforms/xlarge.png", display);
                width = 336.0;
            }
        }

        let mut enemies: Vec<Enemy> = vec![];
        let mut fish: Vec<Fish> = vec![];

        (|| {
            if size == Size::Small {
                return;
            }
            let n = (width / 48.0) as i32;

            let mut e = Enemy::new(display, Species::Land);
            e.set_position(0.0, 24.0);
            enemies.push(e);

            for i in -(n / 2)..=n / 2 {
                let mut f = Fish::new(display);
                f.set_position(i as f32 * 48.0, 36.0);
                fish.push(f);
            }
        })();

        Self {
            width: width,
            height: 96.0,
            x: 0.0,
            y: 0.0,
            size: size,
            texture: texture,
            enemies: enemies,
            enemy_speed: 150.0,
            fish: fish,
            elapsed_time: 0.0,
            platform_type: Type::Plain,
        }
    }

    pub fn set_type(&mut self, t: Type) {
        self.platform_type = t;
        match t {
            Type::Fish => {
                for i in 0..self.fish.len() {
                    self.fish[i].taken = false;
                }
            }
            Type::Enemy => {
                for i in 0..self.enemies.len() {
                    self.enemies[i].set_dead(false);
                }
            }
            _ => {}
        }
    }

    pub fn set_position(&mut self, x: f32, y: f32) {
        self.texture.set_position(x, y);
        self.x = self.texture.x;
        self.y = self.texture.y;
        if self.size == Size::Small {
            return;
        }
        let n = (self.width / 48.0) as usize;
        for i in n / 2..self.fish.len() {
            self.fish[i].set_position(self.x + (i - n / 2) as f32 * 48.0, self.y + 84.0);
        }
        for i in (0..=n / 2).rev() {
            self.fish[i].set_position(self.x + i as f32 * -48.0, self.y + 84.0);
        }
        for i in 0..self.enemies.len() {
            self.enemies[i].set_position(self.x, self.y + 78.0);
        }
    }

    pub fn player_took_fish(&mut self, display: &Display, player: &Player, topbar: &mut Topbar) {
        for i in 0..self.fish.len() {
            if intersect(&self.fish[i].texture, &player.texture) && !self.fish[i].taken {
                topbar.increment_fish_count(display);
                self.fish[i].taken = true;
            }
        }
    }

    pub fn player_vs_enemy(&mut self, display: &Display, player: &mut Player, topbar: &mut Topbar) {
        for i in 0..self.enemies.len() {
            player.check_interaction(&mut self.enemies[i], topbar, display);
        }
    }

    pub fn player_is_on(&mut self, player: &mut Player) -> bool {
        let b = overlap_x(
            Rect {
                x: player.x,
                y: player.y,
                w: player.width,
                h: player.height,
            },
            Rect {
                x: self.x,
                y: self.y,
                w: self.width,
                h: self.height,
            },
        );
        player.set_on_platform(b);

        b
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
        match self.platform_type {
            Type::Enemy => {
                for i in 0..self.enemies.len() {
                    self.enemies[i].update(dt);

                    if self.enemies[i].x + 32.0 >= self.x + self.width / 2.
                        || self.enemies[i].x - 32.0 <= self.x - self.width / 2.
                    {
                        self.enemy_speed *= -1.0;
                        self.enemies[i]
                            .texture
                            .mirror(display, Direction::Horizontal);
                    }
                    self.enemies[i].translate(self.enemy_speed * dt, 0.0);
                }
            }
            Type::Fish => {
                for i in 0..self.fish.len() {
                    if self.elapsed_time > 999. {
                        self.elapsed_time = 1.0;
                    }
                    self.elapsed_time += dt;

                    let t = self.elapsed_time * 1.5;
                    let y = t.sin() * 0.02;

                    if i % 2 == 1 {
                        self.fish[i].translate(0.0, y);
                    } else {
                        self.fish[i].translate(0.0, -y);
                    }
                }
            }
            _ => {}
        }
    }

    pub fn draw(&mut self, target: &mut Frame, program: &Program) {
        self.texture.draw(target, program);
        match self.platform_type {
            Type::Fish => {
                for i in 0..self.fish.len() {
                    self.fish[i].draw(target, program);
                }
            }
            Type::Enemy => {
                for i in 0..self.enemies.len() {
                    self.enemies[i].draw(target, program);
                }
            }
            _ => {}
        }
    }
}
