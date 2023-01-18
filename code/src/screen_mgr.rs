use glium::glutin::event::VirtualKeyCode;
use glium::{Display, Frame, Program};

use crate::game::Game;
use crate::input_mgr::InputManager;
use crate::start_screen::StartScreen;
use crate::background::Background;

enum Screen {
    Start,
    Play,
    GameOver,
}

pub struct ScreenMgr {
    pub game: Game,
    pub start: StartScreen,
    pub input: InputManager,
    current_screen: Screen,
    background: Background,
}

impl ScreenMgr {
    pub fn new(display: &Display) -> Self {
        let game = Game::new(display);
        let start = StartScreen::new(display);
        let input = InputManager::new();
        let background=Background::new(display);

        ScreenMgr {
            game: game,
            start: start,
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
                if self.start.menu_choice==0 && self.input.key_went_up(VirtualKeyCode::Return) {
                    self.current_screen = Screen::Play;
                }
            }
            Screen::Play => {
                self.game.update(&mut self.input, display, dt);
                // if  self.game.game_over(){
                //     self.current_screen=Screen::GameOver;
                // }
            }
            Screen::GameOver=>{

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
            Screen::GameOver=>{

            }
        }
    }
}
