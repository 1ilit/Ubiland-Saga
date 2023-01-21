use glium::glutin::event::VirtualKeyCode;
use glium::{Display, Frame, Program};

use crate::{
    enemy::Enemy,
    gui::Topbar,
    input_mgr::InputManager,
    shape::{BOTTOM, LEFT},
    texture::{AnimatedTexture, AnimationMode, Collide, Transform},
};

pub struct Player {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub texture: AnimatedTexture,
    pub death_animation: AnimatedTexture,
    pub velocity: [f32; 2],
    pub is_on_platform: bool,
    pub is_moving_right: bool,
    pub distance: f32,
    pub is_dead: bool,
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
            death_animation: death,
            x: BOTTOM + 48.0,
            y: 120.0,
            width: width,
            height: height,
            velocity: [0.0, 0.0],
            is_on_platform: false,
            is_moving_right: false,
            distance: 0.0,
            is_dead: false,
        }
    }

    pub fn set_on_platform(&mut self, b: bool) {
        self.is_on_platform = b;
    }

    pub fn is_dead(&self) -> bool {
        self.is_dead
    }

    pub fn set_dead(&mut self, b: bool) {
        self.is_dead = b;
    }

    pub fn reset(&mut self) {
        self.set_dead(false);
        self.x = BOTTOM + 48.0;
        self.y = 120.0;
        self.velocity = [0.0, 0.0];
    }

    pub fn apply_gravity(&mut self, dt: f32) {
        self.y += self.velocity[1];
        if self.y + self.velocity[1] >= BOTTOM - self.height {
            self.velocity[1] -= 3.0 * dt;
        } else {
            self.velocity[1] = 0.0;
        }
        self.texture.set_position(self.x, self.y);
        self.death_animation.set_position(self.x, self.y);
    }

    pub fn was_killed(&mut self, enemy: &Enemy) -> bool {
        (self.texture.collide_left(&enemy.texture) || self.texture.collide_right(&enemy.texture))
            && !enemy.is_dead()
    }

    pub fn check_interaction(&mut self, enemy: &mut Enemy, topbar: &mut Topbar, display: &Display) {
        if self.texture.collide_bottom(&enemy.texture) && !self.is_dead && !enemy.is_dead() {
            topbar.increment_enemy_count(display);
            enemy.set_dead(true);
        } else if self.was_killed(&enemy) {
            self.set_dead(true);
        }
    }

    pub fn update(&mut self, input: &mut InputManager, dt: f32) {
        self.texture.update(dt);
        if self.is_dead() {
            self.is_moving_right = false;
            self.death_animation.update(dt);
            self.apply_gravity(dt);
            return;
        }

        if self.y < BOTTOM - self.height / 2.0 {
            self.set_dead(true);
            return;
        }

        self.x += self.velocity[0];
        self.apply_gravity(dt);

        if input.key_down(VirtualKeyCode::Up) {
            self.velocity[1] = 380.0 * dt;
        }
        if input.key_down(VirtualKeyCode::Right) {
            self.x += 200.0 * dt;
            self.distance += dt;
            self.is_moving_right = true;
        } else {
            self.is_moving_right = false;
        }
        if input.key_down(VirtualKeyCode::Left) && self.is_on_platform {
            self.x -= 200.0 * dt;
            self.distance -= dt;
        }

        if self.x >= 0.0 {
            self.x = 0.0;
        } else if self.x <= LEFT + self.width / 2. {
            self.x = LEFT + self.width / 2.;
        }

        self.texture.set_position(self.x, self.y);
        self.death_animation.set_position(self.x, self.y);
    }

    pub fn draw(&mut self, target: &mut Frame, program: &Program) {
        if self.is_dead {
            self.death_animation.draw(target, program)
        } else {
            self.texture.draw(target, program)
        };
    }
}
