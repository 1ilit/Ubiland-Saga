use glium::{Display, Frame, Program};

use crate::game::Game;
use crate::input_mgr::InputManager;
use crate::start_screen::StartScreen;
use crate::background::Background;

enum CurrentScreen {
    Start,
    Play,
}

pub struct ScreenMgr {
    pub game: Game,
    pub start: StartScreen,
    pub input: InputManager,
    current_screen: CurrentScreen,
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
            current_screen: CurrentScreen::Start,
            background: background,
        }
    }

    pub fn update(&mut self) {
        self.background.update();
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
        self.background.draw(target, program);
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
