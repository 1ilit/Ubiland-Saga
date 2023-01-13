use glium::{Program, Frame, Display};

use crate::texture::{AnimatedTexture, Transform};


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
    pub dead: bool,
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

    pub fn set_x(&mut self, x: f32) {
        self.texture.set_x(x);
        self.x = x;
    }

    pub fn set_y(&mut self, y: f32) {
        self.texture.set_y(y);
        self.y = y;
    }

    pub fn translate(&mut self, x: f32, y: f32) {
        self.texture.translate(x, y);
        self.x = self.texture.x;
        self.y = self.texture.y;
    }
}