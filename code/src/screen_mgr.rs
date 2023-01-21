use glium::glutin::event::VirtualKeyCode;
use glium::{Display, Frame, Program};

use crate::background::Background;
use crate::game::Level;
use crate::input_mgr::InputManager;
use crate::start_screen::StartScreen;
use crate::texture::{Texture, Transform};

enum Screen {
    Start,
    Play,
    GameOver,
    Pause,
}

pub struct Pause {
    pub menu: Texture,
    pub cursor: Texture,
    pub menu_choice: i8,
}

impl Pause {
    pub fn new(display: &Display) -> Self {
        let mut menu = Texture::new("./res/pause_menu.png", display);
        menu.scale(0.7);
        menu.set_position(0.0, 0.0);

        let mut cursor = Texture::new("./res/cursor.png", display);
        cursor.scale(1.5);
        cursor.set_position(0.0, 45.0);

        Self {
            menu: menu,
            cursor: cursor,
            menu_choice: 0,
        }
    }

    pub fn update(&mut self, input: &mut InputManager) {
        if input.key_went_up(VirtualKeyCode::Down) && self.menu_choice < 2 {
            self.menu_choice += 1;
            self.cursor.translate(0., -40.);
        }
        if input.key_went_up(VirtualKeyCode::Up) && self.menu_choice > 0 {
            self.menu_choice -= 1;
            self.cursor.translate(0., 40.);
        }
    }

    pub fn draw(&mut self, target: &mut Frame, program: &Program) {
        self.menu.draw(target, program);
        self.cursor.draw(target, program);
    }
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
    pub level: Level,
    pub start: StartScreen,
    pub game_over: GameOver,
    pub pause: Pause,
    pub input: InputManager,
    current_screen: Screen,
    background: Background,
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
                }
            }
            Screen::Pause => {
                self.pause.update(&mut self.input);
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
