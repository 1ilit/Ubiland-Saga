use glium::glutin::event::VirtualKeyCode;
use glium::{Display, Frame, Program};
use rand::rngs::ThreadRng;
use rand::Rng;

use crate::input_mgr::InputManager;
use crate::shape::{GradientDirection, Rectangle, SCREEN_HEIGHT, SCREEN_WIDTH};
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
    background_clouds: [Texture; 3],
    tex2: Texture,
    logo: Texture,
    rect: Rectangle,
    cursor: Texture,
    menu: Texture,
    pub started: bool,
    pub menu_choice: i8,
    rand: ThreadRng,
}

impl StartScreen {
    pub fn new(display: &Display) -> Self {
        let mut big_cloud1 = Texture::new("./res/big_cloud.png", display);
        big_cloud1.set_position(0., -SCREEN_HEIGHT / 2. + big_cloud1.height / 2.);

        let mut big_cloud2 = Texture::new("./res/big_cloud.png", display);
        big_cloud2.set_position(
            SCREEN_WIDTH - 3.,
            -SCREEN_HEIGHT / 2. + big_cloud2.height / 2.,
        );

        let mut big_cloud3 = Texture::new("./res/rainbow.png", display);
        big_cloud3.set_position(
            SCREEN_WIDTH - 3.,
            -SCREEN_HEIGHT / 2. + big_cloud3.height / 2.,
        );

        let clouds = [big_cloud1, big_cloud2, big_cloud3];

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

        let mut rect = Rectangle::new(display, SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32);
        rect.set_color(display, [0.8, 0.5, 0.3, 1.0]);
        rect.set_gradient(
            display,
            [1.0, 0.45, 1.0, 0.8],
            [0.3, 0.3, 1.0, 0.8],
            GradientDirection::Vertical,
        );
        StartScreen {
            background_clouds: clouds,
            tex2: tex2,
            logo: logo,
            rect: rect,
            started: false,
            cursor: cursor,
            menu: menu,
            menu_choice: 0,
            rand: rand::thread_rng(),
        }
    }

    pub fn update(&mut self, input: &mut InputManager) {
        let (cloud_x, _cloud_y) = self.background_clouds[0].get_position();
        let (cloud_x1, _cloud_y) = self.background_clouds[1].get_position();

        if cloud_x + self.background_clouds[0].width / 2. < (-SCREEN_WIDTH / 2.) {
            self.background_clouds[0].set_position(
                SCREEN_WIDTH - 3.,
                -SCREEN_HEIGHT / 2. + self.background_clouds[0].height / 2.,
            );
        }
        if cloud_x1 + self.background_clouds[1].width / 2. < (-SCREEN_WIDTH / 2.) {
            let x: u8 = self.rand.gen_range(0..4);
            if x == 3 {
                self.background_clouds[2].set_position(
                    SCREEN_WIDTH - 3.,
                    -SCREEN_HEIGHT / 2. + self.background_clouds[0].height / 2.,
                );
            } else {
                self.background_clouds[1].set_position(
                    SCREEN_WIDTH - 3.,
                    -SCREEN_HEIGHT / 2. + self.background_clouds[0].height / 2.,
                );
            }
        }

        for i in 0..3 {
            self.background_clouds[i].translate(-0.02, 0.0);
        }

        if input.key_went_up(VirtualKeyCode::Down) && self.menu_choice < 3 {
            self.menu_choice += 1;
            let (x, y) = self.cursor.get_position();
            self.cursor.set_position(x, y - 35.);
        }
        if input.key_went_up(VirtualKeyCode::Up) && self.menu_choice > 0 {
            self.menu_choice -= 1;
            let (x, y) = self.cursor.get_position();
            self.cursor.set_position(x, y + 35.);
        }
    }

    pub fn draw(&mut self, target: &mut Frame, program: &Program) {
        self.rect.draw(target, program);
        self.background_clouds[0].draw(target, program);

        for i in 0..3 {
            self.background_clouds[i].draw(target, program);
        }

        self.tex2.draw(target, program);
        self.logo.draw(target, program);
        self.menu.draw(target, program);
        self.cursor.draw(target, program);
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
