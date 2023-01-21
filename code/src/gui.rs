use glium::{Display, Frame, Program};
use crate::{texture::{Texture, Transform}, shape::{LEFT, TOP, RIGHT}};

pub struct Score {
    value: u32,
    textures: Vec<Texture>,
}

impl Score {
    pub fn new(display: &Display) -> Self {
        Self {
            value: 0,
            textures: vec![Texture::new("./res/digits/0.png", display)],
        }
    }

    pub fn reset(&mut self, display: &Display) {
        self.value = 0;

        let i = self.textures.len();
        let x = self.textures[i - 1].x;
        let y = self.textures[i - 1].y;
        self.textures.clear();
        self.textures.push(Texture::new("./res/digits/0.png", display));
        self.textures[0].set_position(x, y);
    }

    pub fn increment(&mut self, display: &Display) {
        self.value += 1;
        let mut temp = self.value;
        let mut i = 0;
        while temp > 0 && i < self.textures.len() {
            let (x, y) = (self.textures[i].x, self.textures[i].y);
            if temp % 10 != 0 {
                self.textures[i] =
                    Texture::new(format!("./res/digits/{}.png", temp % 10).as_str(), display);
                self.textures[i].set_position(x, y);
                return;
            } else {
                self.textures[i] = Texture::new("./res/digits/0.png", display);
                self.textures[i].set_position(x, y);
            }
            i += 1;
            temp /= 10;
        }

        let i = self.textures.len();
        let x = self.textures[i - 1].x;
        let y = self.textures[i - 1].y;
        self.textures
            .push(Texture::new("./res/digits/1.png", display));
        let i = self.textures.len();
        self.textures[i - 1].set_position(x, y);

        for i in 0..self.textures.len() - 1 {
            let x = self.textures[i].x;
            let w = self.textures[i].width;
            self.textures[i].set_x(x + w);
        }
    }

    pub fn set_position(&mut self, x: f32, y: f32) {
        self.textures[0].set_position(x, y);
    }

    pub fn draw(&self, target: &mut glium::Frame, program: &glium::Program) {
        for i in 0..self.textures.len() {
            self.textures[i].draw(target, program);
        }
    }
}

pub struct Topbar {
    fish_score: Score,
    fish_label: Texture,
    enemy_score: Score,
    enemy_label: Texture,
    flag_label: Texture,
    distance: Score,
    stop_button: Texture,
}

impl Topbar {
    pub fn new(display: &Display) -> Self {
        let mut fish_label = Texture::new("./res/fish_label.png", display);
        fish_label.set_position(LEFT + 32.0, TOP - 32.0);

        let mut fish_score = Score::new(display);
        fish_score.set_position(LEFT + 80.0, TOP - 32.0);

        let mut enemy_score = Score::new(display);
        enemy_score.set_position(LEFT + 268.0, TOP - 32.0);

        let mut enemy_label = Texture::new("./res/monsta.png", display);
        enemy_label.set_position(LEFT + 220.0, TOP - 30.0);

        let mut flag_label = Texture::new("./res/flag.png", display);
        flag_label.set_position(32.0, TOP - 32.0);

        let mut distance = Score::new(display);
        distance.set_position(80.0, TOP - 32.0);

        let mut stop_button = Texture::new("./res/pause_button.png", display);
        stop_button.set_position(RIGHT - 40.0, TOP - 32.0);

        Self {
            fish_score: fish_score,
            fish_label: fish_label,
            enemy_score: enemy_score,
            enemy_label: enemy_label,
            flag_label: flag_label,
            distance: distance,
            stop_button: stop_button,
        }
    }

    pub fn increment_fish_count(&mut self, display: &Display) {
        self.fish_score.increment(display);
    }

    pub fn increment_enemy_count(&mut self, display: &Display) {
        self.enemy_score.increment(display);
    }

    pub fn increment_distance(&mut self, display: &Display) {
        self.distance.increment(display);
    }

    pub fn reset(&mut self, display: &Display) {
        self.fish_score.reset(display);
        self.enemy_score.reset(display);
        self.distance.reset(display);
    }

    pub fn draw(&mut self, target: &mut Frame, program: &Program) {
        self.fish_score.draw(target, program);
        self.fish_label.draw(target, program);
        self.enemy_label.draw(target, program);
        self.enemy_score.draw(target, program);
        self.flag_label.draw(target, program);
        self.distance.draw(target, program);
        self.stop_button.draw(target, program);
    }
}
