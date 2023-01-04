use glium::{Display, Frame, Program};

use crate::{input_mgr::InputManager, player::Player};

pub struct Game {
    player: Player,
}

impl Game {
    pub fn new(display: &Display) -> Self {
        let p = Player::new(display);

        Game { player: p }
    }

    pub fn update(&mut self, input: &mut InputManager, dt: f32) {
        self.player.update(input, dt);
    }

    pub fn draw(&mut self, target: &mut Frame, program: &Program) {
        self.player.draw(target, program);
    }
}