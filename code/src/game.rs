use glium::glutin::event::VirtualKeyCode;
use glium::{Display, Frame, Program};

use crate::input_mgr::InputManager;
use crate::shape::{GradientDirection, Rectangle, SCREEN_HEIGHT};
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

    pub fn update(&mut self, input: &mut InputManager) {
        self.position[0] += self.velocity[0];
        self.position[1] += self.velocity[1];

        if self.position[1] + self.velocity[1] - self.texture.height / 2.0 > -(SCREEN_HEIGHT / 2.) {
            self.velocity[1] -= 0.4;
        } else {
            self.velocity[1] = 0.0;
        }

        if input.key_down(VirtualKeyCode::Up) {
            self.velocity[1] = 7.;
        }

        self.texture
            .set_position(self.position[0], self.position[1]);
    }

    pub fn draw(&mut self, target: &mut Frame, program: &Program) {
        self.texture.draw(target, program);
    }
}

pub struct StartScreen {
    tex: Texture,
    rect: Rectangle,
    pub started: bool,
}

impl StartScreen {
    pub fn new(display: &Display) -> Self {
        let mut tex = Texture::new("./res/techno.png", display);
        tex.clip(32.0, 0.0, 64.0, 64.0);
        let mut rect = Rectangle::new(display, 300, 300);
        rect.set_color(display, [0.8, 0.5, 0.3, 1.0]);
        rect.set_gradient( 
            display,
            [0.0, 0.0, 1.0, 1.0],
            [0.0, 1.0, 0.0, 1.0],
            GradientDirection::Horizontal,
        );
        rect.set_position(100., 100.);
        StartScreen {
            tex: tex,
            rect: rect,
            started: false,
        }
    }

    pub fn update(&mut self, input: &mut InputManager) {
        if input.key_went_up(VirtualKeyCode::Return) {
            self.started = true;
        }
    }

    pub fn draw(&mut self, target: &mut Frame, program: &Program) {
        self.rect.draw(target, program);
        self.tex.draw(target, program);
    }
}

pub struct Game {
    player: Player,
}

impl Game {
    pub fn new(display: &Display) -> Self {
        let p = Player::new(display);

        Game { player: p }
    }

    pub fn update(&mut self, input: &mut InputManager) {
        self.player.update(input);
    }

    pub fn draw(&mut self, target: &mut Frame, program: &Program) {
        self.player.draw(target, program);
    }
}

enum CurrentScreen {
    Start,
    Play,
}

pub struct ScreenMgr {
    pub game: Game,
    pub start: StartScreen,
    pub input: InputManager,
    current_screen: CurrentScreen,
}

impl ScreenMgr {
    pub fn new(display: &Display) -> Self {
        let game = Game::new(display);
        let start = StartScreen::new(display);
        let input = InputManager::new();

        ScreenMgr {
            game: game,
            start: start,
            input: input,
            current_screen: CurrentScreen::Start,
        }
    }

    pub fn update(&mut self) {
        match self.current_screen {
            CurrentScreen::Start => {
                self.start.update(&mut self.input);
                if self.start.started {
                    self.current_screen = CurrentScreen::Play;
                }
            }
            CurrentScreen::Play => {
                self.game.update(&mut self.input);
            }
        }
    }

    pub fn draw(&mut self, target: &mut Frame, program: &Program) {
        match self.current_screen {
            CurrentScreen::Start => {
                self.start.draw(target, program);
            }
            CurrentScreen::Play => {
                self.game.draw(target, program);
            }
        }
    }
}
