use glium::glutin::event::VirtualKeyCode;
use glium::{Display, Frame, Program};

use crate::input_mgr::InputManager;
use crate::shape::{SCREEN_WIDTH};
use crate::texture::Texture;

pub struct StartScreen {
    tex2: Texture,
    logo: Texture,
    cursor: Texture,
    menu: Texture,
    pub started: bool,
    pub menu_choice: i8,
}

impl StartScreen {
    pub fn new(display: &Display) -> Self {
        let mut tex2 = Texture::new("./res/grass_tileset.png", display);
        tex2.set_position(-200., -150.);

        let mut logo = Texture::new("./res/logo.png", display);
        logo.scale(1.2);
        logo.set_position(SCREEN_WIDTH / 3. - logo.width / 2. + 50., 70.);

        let mut menu = Texture::new("./res/menu.png", display);
        menu.scale(1.1);
        menu.set_position(SCREEN_WIDTH / 3. - menu.width / 2., -90.);

        let mut cursor = Texture::new("./res/cursor.png", display);
        cursor.scale(1.1);
        cursor.set_position(SCREEN_WIDTH / 3. - cursor.width / 2. + 15., -35.);

        StartScreen {
            tex2: tex2,
            logo: logo,
            started: false,
            cursor: cursor,
            menu: menu,
            menu_choice: 0,
        }
    }

    pub fn update(&mut self, input: &mut InputManager) {
        if input.key_went_up(VirtualKeyCode::Down) && self.menu_choice < 3 {
            self.menu_choice += 1;
            self.cursor.translate(0., -35.);
        }
        if input.key_went_up(VirtualKeyCode::Up) && self.menu_choice > 0 {
            self.menu_choice -= 1;
            self.cursor.translate(0., 35.);
        }
    }

    pub fn draw(&mut self, target: &mut Frame, program: &Program) {
        self.tex2.draw(target, program);
        self.logo.draw(target, program);
        self.menu.draw(target, program);
        self.cursor.draw(target, program);
    }
}