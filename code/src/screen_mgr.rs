use glium::{Display, Frame, Program};

use crate::game::Game;
use crate::input_mgr::InputManager;
use crate::start_screen::StartScreen;

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
