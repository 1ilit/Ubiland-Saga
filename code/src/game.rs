use std::vec;

use glium::{Display, Frame, Program};

use crate::{
    input_mgr::InputManager,
    player::Player,
    shape::{Direction, LEFT, SCREEN_WIDTH},
    texture::{AnimatedTexture, Texture, Transform, Collide},
};

#[derive(Debug, PartialEq)]
pub enum Size {
    Small,
    Medium,
    Large,
    XLarge,
}

impl Size {
    fn from_u32(value: u32) -> Size {
        match value {
            0 => Size::Small,
            1 => Size::Medium,
            2 => Size::Large,
            _ => panic!("Unknown value: {}", value),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Type {
    Enemy,
    Fish,
}

#[derive(Debug, PartialEq)]
pub enum Species {
    Land,
    Flying,
}

pub struct Enemy {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    texture: AnimatedTexture,
    speed: f32,
    dead: bool,
}

impl Enemy {
    pub fn new(display: &Display, species: Species) -> Self {
        let texture: AnimatedTexture;
        match species {
            Species::Land => {
                texture = AnimatedTexture::new(
                    display,
                    vec!["./res/enemy1.png", "./res/enemy2.png"],
                    0.3,
                    2,
                );
            }
            Species::Flying => {
                texture = AnimatedTexture::new(
                    display,
                    vec!["./res/enemy1.png", "./res/enemy2.png"],
                    0.3,
                    2,
                );
            }
        }
        let (width, height) = texture.get_dimensions();
        Self {
            x: 0.0,
            y: 0.0,
            width: width,
            height: height,
            speed: 200.0,
            texture: texture,
            dead: false,
        }
    }

    pub fn set_position(&mut self, x: f32, y: f32) {
        self.texture.set_position(x, y);
        self.x = x;
        self.y = y;
    }

    pub fn update(&mut self, dt: f32) {
        self.texture.update(dt);
    }

    pub fn draw(&mut self, target: &mut Frame, program: &Program) {
        self.texture.draw(target, program);
    }

    fn set_x(&mut self, x: f32) {
        self.texture.set_x(x);
        self.x = x;
    }

    fn set_y(&mut self, y: f32) {
        self.texture.set_y(y);
        self.y = y;
    }

    fn translate(&mut self, x: f32, y: f32) {
        self.texture.translate(x, y);
        self.x = self.texture.x;
        self.y = self.texture.y;
    }
}

pub struct Platform {
    width: f32,
    height: f32,
    x: f32,
    y: f32,
    size: Size,
    texture: Texture,
    enemies: Vec<Enemy>,
    enemy_speed: f32,
    fish: Vec<Texture>,
    elapsed_time: f32,
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
            size: size,
            texture: texture,
            enemies: vec![],
            enemy_speed: 150.0,
            fish: vec![],
            elapsed_time: 0.0,
        }
    }

    pub fn spawn_entity(&mut self, display: &Display, t: Type, n: u8) {
        if self.size == Size::Small {
            return;
        }

        match t {
            Type::Enemy => {
                let mut e = Enemy::new(display, Species::Land);
                e.set_position(0.0, 24.0);
                self.enemies.push(e);
            }
            Type::Fish => {
                for i in (1..=n / 2).rev() {
                    let mut f = Texture::new("./res/fish.png", display);
                    f.set_position(i as f32 * -48.0, 36.0);
                    self.fish.push(f);
                }
                for i in 0..=n / 2 {
                    let mut f = Texture::new("./res/fish.png", display);
                    f.set_position(i as f32 * 48.0, 36.0);
                    self.fish.push(f);
                }
            }
        };
    }

    pub fn despawn_entity(&mut self) {
        self.enemies.clear();
        self.fish.clear();
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
            self.enemies[i].set_y(self.y + 80.0);
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
                self.enemies[i]
                    .texture
                    .mirror(display, Direction::Horizontal);
            }
            self.enemies[i].translate(self.enemy_speed * dt, 0.0);
        }

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
    controls: Vec<Texture>,
    elapsed_time: f32,
}

impl Game {
    pub fn new(display: &Display) -> Self {
        let p = Player::new(display);

        let mut platforms: Vec<Platform> = vec![];

        let mut starting_platform = Platform::new(display, Size::XLarge);
        starting_platform.set_position(LEFT + 100.0, -50.0);
        platforms.push(starting_platform);

        for i in 0..9 {
            platforms.push(Platform::new(display, Size::from_u32(i % 3)));
        }

        for i in 1..4 {
            let w = (platforms[i].width / 48.0) as u8;
            platforms[i].spawn_entity(display, Type::Fish, w);
        }

        for i in 4..7 {
            let w = (platforms[i].width / 48.0) as u8;
            platforms[i].spawn_entity(display, Type::Enemy, w);
        }

        for i in 1..10 {
            platforms[i].set_position(-SCREEN_WIDTH, 0.0);
        }

        platforms[9].set_position(510.0, -100.0);

        let mut controls: Vec<Texture> = vec![];
        let mut c1 = Texture::new("./res/controls1.png", display);
        c1.scale(0.8);
        c1.set_position(-210.0, 160.0);
        controls.push(c1);
        let mut c2 = Texture::new("./res/controls2.png", display);
        c2.scale(0.8);
        c2.set_position(510.0, 160.0);
        controls.push(c2);

        Game {
            player: p,
            platforms: platforms,
            controls: controls,
            elapsed_time: 0.0,
        }
    }

    pub fn update(&mut self, input: &mut InputManager, display: &Display, dt: f32) {
        self.player.update(input, dt);
        for i in 0..self.platforms.len() {
            self.platforms[i].update(display, dt);
        }

        for i in 0..self.platforms.len() {
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

        for i in 0..self.controls.len(){
            if self.elapsed_time > 999. {
                self.elapsed_time = 1.0;
            }
            self.elapsed_time += dt;

            let t = self.elapsed_time * 1.5;
            let y = t.sin() * 0.04;

            self.controls[i].translate(0.0, y);
        }

        if self.player.right {
            for i in 0..self.platforms.len() {
                self.platforms[i].translate(-80.0 * dt, 0.0);
            }
            for i in 0..self.controls.len(){
                self.controls[i].translate(-80.0 * dt, 0.0);
            }
        }
    }

    pub fn draw(&mut self, target: &mut Frame, program: &Program) {
        for i in 0..self.controls.len() {
            self.controls[i].draw(target, program);
        }
        for i in 0..self.platforms.len() {
            self.platforms[i].draw(target, program);
        }

        self.player.draw(target, program);
    }
}
