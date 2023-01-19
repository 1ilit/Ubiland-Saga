use glium::glutin::event::VirtualKeyCode;
use glium::{Display, Frame, Program};

use crate::input_mgr::InputManager;
use crate::shape::{BOTTOM, LEFT};
use crate::texture::{AnimatedTexture, AnimationMode, Transform};

pub struct Player {
    pub texture: AnimatedTexture,
    pub death: AnimatedTexture,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub velocity: [f32; 2],
    pub on_platform: bool,
    pub right: bool,
    pub distance: f32,
    pub dead: bool,
}

impl Player {
    pub fn new(display: &Display) -> Self {
        let texture = AnimatedTexture::new(
            display,
            vec![
                "./res/ubi1.png",
                "./res/ubi2.png",
                "./res/ubi3.png",
                "./res/ubi4.png",
            ],
            0.2,
            3,
        );

        let mut death = AnimatedTexture::new(
            display,
            vec![
                "./res/ubi_death_1.png",
                "./res/ubi_death_2.png",
                "./res/ubi_death_3.png",
                "./res/ubi_death_4.png",
                "./res/ubi_death_5.png",
            ],
            0.15,
            5,
        );
        death.set_mode(AnimationMode::Once);

        let (width, height) = texture.get_dimensions();
        Player {
            texture: texture,
            death: death,
            x: BOTTOM + 48.0,
            y: 120.0,
            width: width,
            height: height,
            velocity: [0.0, 0.0],
            on_platform: false,
            right: false,
            distance: 0.0,
            dead: false,
        }
    }

    pub fn apply_gravity(&mut self, dt: f32) {
        self.y += self.velocity[1];
        if self.y + self.velocity[1] >= BOTTOM - self.height {
            self.velocity[1] -= 3.0 * dt;
        } else {
            self.velocity[1] = 0.0;
        }
        self.texture.set_position(self.x, self.y);
        self.death.set_position(self.x, self.y);
    }

    pub fn update(&mut self, input: &mut InputManager, dt: f32) {
        if self.dead {
            self.right = false;
            self.death.update(dt);
            self.apply_gravity(dt);
            return;
        }
        self.texture.update(dt);
        self.x += self.velocity[0];

        self.apply_gravity(dt);

        if input.key_down(VirtualKeyCode::Up) {
            self.velocity[1] = 380.0 * dt;
        }
        if input.key_down(VirtualKeyCode::Right) {
            self.x += 200.0 * dt;
            self.distance += dt;
            self.right = true;
        } else {
            self.right = false;
        }
        if input.key_down(VirtualKeyCode::Left) && self.on_platform {
            self.x -= 200.0 * dt;
            self.distance -= dt;
        }

        if self.x >= 0.0 {
            self.x = 0.0;
        } else if self.x <= LEFT + self.width / 2. {
            self.x = LEFT + self.width / 2.;
        }

        self.texture.set_position(self.x, self.y);
        self.death.set_position(self.x, self.y);
    }

    pub fn draw(&mut self, target: &mut Frame, program: &Program) {
        if !self.dead {
            self.texture.draw(target, program);
        } else {
            self.death.draw(target, program);
        }
    }
}
