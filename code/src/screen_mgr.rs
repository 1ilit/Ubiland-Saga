use glium::glutin::event::VirtualKeyCode;
use glium::{Display, Frame, Program};

use crate::background::Background;
use crate::game::Game;
use crate::input_mgr::InputManager;
use crate::shape::SCREEN_WIDTH;
use crate::start_screen::StartScreen;
use crate::texture::{Texture, Transform};

enum Screen {
    Start,
    Play,
    GameOver,
}

pub struct GameOver {
    pub texture: Texture,
    pub menu: Texture,
    pub cursor: Texture,
    pub menu_choice: i8,
    pub elapsed_time: f32,
}

impl GameOver {
    pub fn new(display: &Display) -> Self {
        let mut texture = Texture::new("./res/game_over.png", display);
        texture.set_y(100.0);

        let mut menu = Texture::new("./res/game_over_menu.png", display);
        menu.scale(0.7);
        menu.set_position(0.0, -30.0);

        let mut cursor = Texture::new("./res/cursor.png", display);
        cursor.scale(1.2);
        cursor.set_position(0.0, -5.0);

        Self {
            texture: texture,
            menu: menu,
            cursor: cursor,
            menu_choice: 0,
            elapsed_time: 0.0,
        }
    }

    pub fn update(&mut self, input: &mut InputManager, dt: f32) {
        if self.elapsed_time > 99999. {
            self.elapsed_time = 1.0;
        }

        self.elapsed_time += dt;

        let t = self.elapsed_time * 3.5;

        let y = t.sin() * 0.015;
        self.texture.translate(0.0, y);

        if input.key_went_up(VirtualKeyCode::Down) && self.menu_choice < 1 {
            self.menu_choice += 1;
            self.cursor.translate(0., -45.);
        }
        if input.key_went_up(VirtualKeyCode::Up) && self.menu_choice > 0 {
            self.menu_choice -= 1;
            self.cursor.translate(0., 45.);
        }
    }

    pub fn draw(&mut self, target: &mut Frame, program: &Program) {
        self.texture.draw(target, program);
        self.menu.draw(target, program);
        self.cursor.draw(target, program);
    }
}

pub struct ScreenMgr {
    pub game: Game,
    pub start: StartScreen,
    pub game_over: GameOver,
    pub input: InputManager,
    current_screen: Screen,
    background: Background,
}

impl ScreenMgr {
    pub fn new(display: &Display) -> Self {
        let game = Game::new(display);
        let start = StartScreen::new(display);
        let input = InputManager::new();
        let background = Background::new(display);

        ScreenMgr {
            game: game,
            start: start,
            game_over: GameOver::new(display),
            input: input,
            current_screen: Screen::Start,
            background: background,
        }
    }

    pub fn update(&mut self, display: &Display, dt: f32) {
        self.background.update(dt);
        match self.current_screen {
            Screen::Start => {
                self.start.update(&mut self.input, dt);
                if self.start.menu_choice == 0 && self.input.key_went_up(VirtualKeyCode::Return) {
                    self.current_screen = Screen::Play;
                }
            }
            Screen::Play => {
                self.game.update(&mut self.input, display, dt);
                if self.game.game_over(dt) {
                    self.current_screen = Screen::GameOver;
                }
            }
            Screen::GameOver => {
                self.game_over.update(&mut self.input, dt);
                if self.game_over.menu_choice == 0 && self.input.key_went_up(VirtualKeyCode::Return)
                {
                    self.current_screen = Screen::Play;
                    self.game.restart(display);
                } else if self.game_over.menu_choice == 1
                    && self.input.key_went_up(VirtualKeyCode::Return)
                {
                    self.current_screen = Screen::Start;
                }
            }
        }
    }

    pub fn draw(&mut self, target: &mut Frame, program: &Program) {
        self.background.draw(target, program);
        match self.current_screen {
            Screen::Start => {
                self.start.draw(target, program);
            }
            Screen::Play => {
                self.game.draw(target, program);
            }
            Screen::GameOver => {
                self.game_over.draw(target, program);
            }
        }
    }
}
