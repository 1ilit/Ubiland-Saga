use glium::{Display, Frame, Program};

use crate::{
    shape::BOTTOM,
    texture::{AnimatedTexture, AnimationMode, Transform},
};

pub const SPAWN_DELAY: f32 = 30.0;

#[derive(Debug, PartialEq)]
pub enum Species {
    Land,
    Flying,
}

pub struct Enemy {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub texture: AnimatedTexture,
    pub speed: f32,
    pub is_dead: bool,
    pub death_animation: AnimatedTexture,
    pub y_velocity: f32,
}

impl Enemy {
    pub fn new(display: &Display, species: Species) -> Self {
        let texture: AnimatedTexture;
        let mut death: AnimatedTexture;
        match species {
            Species::Land => {
                texture = AnimatedTexture::new(
                    display,
                    vec!["./res/enemy/land_enemy_1.png", "./res/enemy/land_enemy_2.png"],
                    0.3,
                    2,
                );
                death = AnimatedTexture::new(
                    display,
                    vec![
                        "./res/enemy/land_enemy_death_1.png",
                        "./res/enemy/land_enemy_death_2.png",
                        "./res/enemy/land_enemy_death_3.png",
                    ],
                    0.35,
                    3,
                );
                death.set_mode(AnimationMode::Once);
            }
            Species::Flying => {
                texture = AnimatedTexture::new(
                    display,
                    vec![
                        "./res/enemy/flying_enemy_1.png",
                        "./res/enemy/flying_enemy_2.png",
                        "./res/enemy/flying_enemy_3.png",
                        "./res/enemy/flying_enemy_4.png",
                        "./res/enemy/flying_enemy_3.png",
                        "./res/enemy/flying_enemy_2.png",
                        "./res/enemy/flying_enemy_1.png",
                    ],
                    0.15,
                    7,
                );
                death = AnimatedTexture::new(
                    display,
                    vec![
                        "./res/enemy/flying_enemy_death_1.png",
                        "./res/enemy/flying_enemy_death_2.png",
                        "./res/enemy/flying_enemy_death_3.png",
                    ],
                    0.35,
                    3,
                );
                death.set_mode(AnimationMode::Once);
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
            is_dead: false,
            death_animation: death,
            y_velocity: 0.0,
        }
    }

    pub fn set_position(&mut self, x: f32, y: f32) {
        self.texture.set_position(x, y);
        self.death_animation.set_position(x, y);
        self.x = x;
        self.y = y;
    }

    pub fn set_x(&mut self, x: f32) {
        self.texture.set_x(x);
        self.death_animation.set_x(x);
        self.x = x;
    }

    pub fn _set_y(&mut self, y: f32) {
        self.texture.set_y(y);
        self.death_animation.set_y(y);
        self.y = y;
    }

    pub fn is_dead(&self) -> bool {
        self.is_dead
    }

    pub fn set_dead(&mut self, b: bool) {
        self.is_dead = b;
    }

    pub fn translate(&mut self, x: f32, y: f32) {
        if !self.is_dead {
            self.texture.translate(x, y);
            self.death_animation.translate(x, y);
            self.x = self.texture.x;
            self.y = self.texture.y;
        }
    }

    pub fn apply_gravity(&mut self, dt: f32) {
        self.y += self.y_velocity;
        if self.y + self.y_velocity >= BOTTOM - self.height {
            self.y_velocity -= 3.0 * dt;
        } else {
            self.y_velocity = 0.0;
        }
        self.set_position(self.x, self.y);
    }

    pub fn update(&mut self, dt: f32) {
        if !self.is_dead {
            self.texture.update(dt);
        } else {
            self.death_animation.update(dt);
        }
    }

    pub fn draw(&mut self, target: &mut Frame, program: &Program) {
        if !self.is_dead {
            self.texture.draw(target, program);
        } else {
            self.death_animation.draw(target, program);
        }
    }
}
