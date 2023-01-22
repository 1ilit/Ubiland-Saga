use glium::glutin::event::VirtualKeyCode;
use glium::{Display, Frame, Program};

use crate::input_mgr::InputManager;
use crate::platform::{Platform, Size};
use crate::shape::SCREEN_WIDTH;
use crate::texture::{AnimatedTexture, Texture, Transform};

pub enum Screen {
    Start,
    Play,
    GameOver,
    Pause,
    Store,
}

pub struct StartScreen {
    logo: Texture,
    cursor: Texture,
    menu: Texture,
    platform: Platform,
    platform2: Platform,
    ubi: AnimatedTexture,
    pub started: bool,
    pub menu_choice: i8,
    elapsed_time: f32,
}

impl StartScreen {
    pub fn new(display: &Display) -> Self {
        let mut logo = Texture::new("./res/gui/logo.png", display);
        logo.scale(1.2);
        logo.set_position(SCREEN_WIDTH / 3. - logo.width / 2. + 50., 70.);

        let mut menu = Texture::new("./res/gui/start_menu.png", display);
        menu.set_position(160.0, -85.0);

        let mut cursor = Texture::new("./res/gui/cursor.png", display);
        cursor.set_position(SCREEN_WIDTH / 3. - cursor.width / 2. + 5.0, -45.0);

        let mut platform = Platform::new(display, Size::Medium);
        platform.set_position(-170.0, -60.0);

        let mut platform2 = Platform::new(display, Size::Large);
        platform2.set_position(-350.0, -200.0);

        let mut ubi = AnimatedTexture::new(
            display,
            vec![
                "./res/player/loaf1.png",
                "./res/player/loaf2.png",
                "./res/player/loaf3.png",
                "./res/player/loaf4.png",
            ],
            0.2,
            4,
        );
        let h = ubi.height / 2.0;
        ubi.set_position(-180.0, -12.0 + h);

        StartScreen {
            logo: logo,
            started: false,
            cursor: cursor,
            platform: platform,
            platform2: platform2,
            ubi: ubi,
            menu: menu,
            menu_choice: 0,
            elapsed_time: 0.0,
        }
    }

    pub fn update(&mut self, input: &mut InputManager, dt: f32) {
        self.ubi.update(dt);

        if self.elapsed_time > 999. {
            self.elapsed_time = 1.0;
        }

        self.elapsed_time += dt;

        let t = self.elapsed_time * 3.5;

        let y = t.sin() * 0.015;
        self.logo.translate(0.0, y);

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
        self.platform2.draw(target, program);
        self.logo.draw(target, program);
        self.menu.draw(target, program);
        self.cursor.draw(target, program);
        self.platform.draw(target, program);
        self.ubi.draw(target, program);
    }
}

pub struct Store {
    panel: Texture,
    coming_soon: Texture,
    elapsed_time: f32,
    exited: bool,
}

impl Store {
    pub fn new(display: &Display) -> Self {
        Self {
            panel: Texture::new("./res/gui/panel.png", display),
            coming_soon: Texture::new("./res/gui/coming_soon.png", display),
            elapsed_time: 0.0,
            exited: false,
        }
    }

    pub fn exited(&mut self) -> bool {
        let temp = self.exited;
        self.exited = false;
        temp
    }

    pub fn update(&mut self, input: &mut InputManager, dt: f32) {
        if input.key_went_up(VirtualKeyCode::Escape) {
            self.exited = true;
        }

        if self.elapsed_time > 99999. {
            self.elapsed_time = 1.0;
        }

        self.elapsed_time += dt;

        let t = self.elapsed_time * 3.5;

        let y = t.sin() * 0.015;
        self.coming_soon.translate(0.0, y);
    }

    pub fn draw(&mut self, target: &mut Frame, program: &Program) {
        self.panel.draw(target, program);
        self.coming_soon.draw(target, program);
    }
}

pub struct Pause {
    menu: Texture,
    cursor: Texture,
    panel: Texture,
    pub menu_choice: i8,
    title: Texture,
    elapsed_time: f32,
}

impl Pause {
    pub fn new(display: &Display) -> Self {
        let mut menu = Texture::new("./res/gui/pause_menu.png", display);
        menu.set_position(0.0, -50.0);

        let mut cursor = Texture::new("./res/gui/cursor.png", display);
        cursor.scale(1.3);
        cursor.set_position(0.0, 10.0);

        let mut title = Texture::new("./res/gui/game_paused.png", display);
        title.set_position(0.0, 80.0);

        Self {
            menu: menu,
            cursor: cursor,
            panel: Texture::new("./res/gui/panel.png", display),
            menu_choice: 0,
            title: title,
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
        self.title.translate(0.0, y);

        if input.key_went_up(VirtualKeyCode::Down) && self.menu_choice < 3 {
            self.menu_choice += 1;
            self.cursor.translate(0., -40.);
        }
        if input.key_went_up(VirtualKeyCode::Up) && self.menu_choice > 0 {
            self.menu_choice -= 1;
            self.cursor.translate(0., 40.);
        }
    }

    pub fn draw(&mut self, target: &mut Frame, program: &Program) {
        self.panel.draw(target, program);
        self.menu.draw(target, program);
        self.title.draw(target, program);
        self.cursor.draw(target, program);
    }
}

pub struct GameOver {
    texture: Texture,
    menu: Texture,
    cursor: Texture,
    pub menu_choice: i8,
    elapsed_time: f32,
    panel: Texture,
}

impl GameOver {
    pub fn new(display: &Display) -> Self {
        let mut texture = Texture::new("./res/gui/game_over.png", display);
        texture.set_y(80.0);

        let mut menu = Texture::new("./res/gui/game_over_menu.png", display);
        menu.set_position(0.0, -50.0);

        let mut cursor = Texture::new("./res/gui/cursor.png", display);
        cursor.set_position(0.0, -10.0);

        Self {
            texture: texture,
            menu: menu,
            cursor: cursor,
            menu_choice: 0,
            elapsed_time: 0.0,
            panel: Texture::new("./res/gui/panel.png", display),
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
        self.panel.draw(target, program);
        self.texture.draw(target, program);
        self.menu.draw(target, program);
        self.cursor.draw(target, program);
    }
}
