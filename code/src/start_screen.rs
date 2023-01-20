use glium::glutin::event::VirtualKeyCode;
use glium::{Display, Frame, Program};

use crate::input_mgr::InputManager;
use crate::platform::{Platform, Size};
use crate::shape::SCREEN_WIDTH;
use crate::texture::{AnimatedTexture, Texture, Transform};

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
        let mut logo = Texture::new("./res/logo.png", display);
        logo.scale(1.2);
        logo.set_position(SCREEN_WIDTH / 3. - logo.width / 2. + 50., 70.);

        let mut menu = Texture::new("./res/menu.png", display);
        menu.scale(0.7);
        menu.set_position(160.0, -35.0);

        let mut cursor = Texture::new("./res/cursor.png", display);
        cursor.scale(1.1);
        cursor.set_position(SCREEN_WIDTH / 3. - cursor.width / 2. + 15., -35.);

        let mut platform = Platform::new(display, Size::Medium);
        platform.set_position(-170.0, -60.0);

        let mut platform2 = Platform::new(display, Size::Large);
        platform2.set_position(-350.0, -200.0);

        let mut ubi = AnimatedTexture::new(
            display,
            vec![
                "./res/loaf1.png",
                "./res/loaf2.png",
                "./res/loaf3.png",
                "./res/loaf4.png",
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

        if input.key_went_up(VirtualKeyCode::Down) && self.menu_choice < 0 {
            self.menu_choice += 1;
            self.cursor.translate(0., -35.);
        }
        if input.key_went_up(VirtualKeyCode::Up) && self.menu_choice > 0 {
            self.menu_choice -= 1;
            self.cursor.translate(0., 35.);
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
