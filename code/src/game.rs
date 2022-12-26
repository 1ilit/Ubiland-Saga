use glium::{Display, Frame, Program};

use crate::input_mgr::InputManager;
use crate::texture::Texture;

pub struct Player {
    pub texture: Texture,
    pub position: [f32; 2],
    pub velocity: [f32; 2],
}

impl Player {
    pub fn new(display: &Display) -> Self {
        let texture = Texture::new("./res/star.png", display);
        Player {
            texture: texture,
            position: [0.0, 0.0],
            velocity: [0.0, 0.0],
        }
    }

    pub fn update(&mut self) {
        self.position[0] += self.velocity[0];
        self.position[1] += self.velocity[1];

        if self.position[1] + self.velocity[1] - self.texture.height / 2.0 > -300.0 {
            self.velocity[1] -= 0.4;
        } else {
            self.velocity[1] = 0.0;
        }

        self.texture
            .set_position(self.position[0], self.position[1]);
    }

    pub fn draw(&mut self, target: &mut Frame, program: &Program) {
        self.texture.draw(target, program);
    }
}

pub struct Game {
    player: Player,
    pub input: InputManager,
}

impl Game {
    pub fn new(display: &Display) -> Self {
        let p = Player::new(display);
        let input = InputManager::new();

        Game {
            player: p,
            input: input,
        }
    }

    pub fn update(&mut self) {
        if self
            .input
            .key_went_up(glium::glutin::event::VirtualKeyCode::Up)
        {
            println!("up is pressed");
            self.player.texture.set_position(60.0, 60.0);
        }
        self.player.update();
    }

    pub fn draw(&mut self, target: &mut Frame, program: &Program) {
        self.player.draw(target, program);
    }
}
