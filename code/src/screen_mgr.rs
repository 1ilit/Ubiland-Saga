use glium::glutin::event::VirtualKeyCode;
use glium::{Display, Frame, Program};

use crate::background::Background;
use crate::game::Level;
use crate::input_mgr::InputManager;
use crate::screens::{GameOver, Pause, Screen, StartScreen, Store};

pub struct ScreenMgr {
    pub level: Level,
    pub start: StartScreen,
    pub game_over: GameOver,
    pub pause: Pause,
    pub store: Store,
    pub input: InputManager,
    current_screen: Screen,
    background: Background,
    exit: bool,
}

impl ScreenMgr {
    pub fn new(display: &Display) -> Self {
        let level = Level::new(display);
        let start = StartScreen::new(display);
        let input = InputManager::new();
        let background = Background::new(display);

        ScreenMgr {
            level: level,
            start: start,
            game_over: GameOver::new(display),
            pause: Pause::new(display),
            store: Store::new(display),
            input: input,
            current_screen: Screen::Start,
            background: background,
            exit: false,
        }
    }

    pub fn exited(&self) -> bool {
        self.exit
    }

    pub fn update(&mut self, display: &Display, dt: f32) {
        self.background.update(dt);
        match self.current_screen {
            Screen::Start => {
                self.start.update(&mut self.input, dt);
                if self.start.menu_choice == 0 && self.input.key_went_up(VirtualKeyCode::Return) {
                    self.current_screen = Screen::Play;
                } else if self.start.menu_choice == 1 && self.input.key_went_up(VirtualKeyCode::Return) {
                    self.current_screen = Screen::Store;
                }else if self.start.menu_choice == 2 && self.input.key_went_up(VirtualKeyCode::Return) {
                    self.exit=true;
                }
            }
            Screen::Store=>{
                self.store.update(&mut self.input, dt);
                if self.store.exited(){
                    self.current_screen=Screen::Start;
                }
            }
            Screen::Play => {
                self.level.update(&mut self.input, display, dt);
                if self.level.game_over(dt) {
                    self.current_screen = Screen::GameOver;
                }
                if self.level.paused() {
                    self.current_screen = Screen::Pause;
                }
            }
            Screen::GameOver => {
                self.game_over.update(&mut self.input, dt);
                if self.game_over.menu_choice == 0 && self.input.key_went_up(VirtualKeyCode::Return)
                {
                    self.current_screen = Screen::Play;
                    self.level.restart(display);
                } else if self.game_over.menu_choice == 1
                    && self.input.key_went_up(VirtualKeyCode::Return)
                {
                    self.current_screen = Screen::Start;
                } else if self.game_over.menu_choice == 2
                    && self.input.key_went_up(VirtualKeyCode::Return)
                {
                    self.exit = true;
                }
            }
            Screen::Pause => {
                self.pause.update(&mut self.input, dt);
                if self.pause.menu_choice == 0 && self.input.key_went_up(VirtualKeyCode::Return) {
                    self.current_screen = Screen::Play;
                    self.level.resume();
                } else if self.pause.menu_choice == 1
                    && self.input.key_went_up(VirtualKeyCode::Return)
                {
                    self.current_screen = Screen::Play;
                    self.level.restart(display);
                    self.level.resume();
                } else if self.pause.menu_choice == 2
                    && self.input.key_went_up(VirtualKeyCode::Return)
                {
                    self.current_screen = Screen::Start;
                    self.level.resume();
                } else if self.pause.menu_choice == 3
                    && self.input.key_went_up(VirtualKeyCode::Return)
                {
                    self.exit = true;
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
            Screen::Store=>{
                self.store.draw(target, program);
            }
            Screen::Play => {
                self.level.draw(target, program);
            }
            Screen::GameOver => {
                self.game_over.draw(target, program);
            }
            Screen::Pause => {
                self.pause.draw(target, program);
            }
        }
    }
}
