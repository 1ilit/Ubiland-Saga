use std::vec;

use glium::{glutin::event::VirtualKeyCode, Display, Frame, Program};
use rand::{rngs::ThreadRng, Rng};

use crate::{
    enemy::{Enemy, Species, SPAWN_DELAY},
    input_mgr::InputManager,
    platform::{Platform, Size, Type},
    player::Player,
    shape::{BOTTOM, LEFT, RIGHT, SCREEN_WIDTH, TOP},
    texture::{AnimatedTexture, Collide, Rect, Score, Texture, Transform},
};

fn overlap_x(a: Rect, b: Rect) -> bool {
    if a.x + a.w / 2.0 <= b.x - b.w / 2.0 {
        return false;
    }

    if a.x - a.w / 2.0 >= b.x + b.w / 2.0 {
        return false;
    }

    true
}

fn intersect(a: &Texture, b: &AnimatedTexture) -> bool {
    if a.y - a.height / 2.0 >= b.y + b.height / 2.0 {
        return false;
    }

    if a.y + a.height / 2.0 <= b.y - b.height / 2.0 {
        return false;
    }

    if a.x + a.width / 2.0 <= b.x - b.width / 2.0 {
        return false;
    }

    if a.x - a.width / 2.0 >= b.x + b.width / 2.0 {
        return false;
    }

    true
}

fn player_landed(player: &Player, platform: &Platform) -> bool {
    player.x + player.width / 2. >= platform.x - platform.width / 2.
        && player.x - player.width / 2. <= platform.x + platform.width / 2.
        && player.y - player.height / 2. + player.velocity[1] <= platform.y + platform.height / 2.
        && player.y - player.height / 2. >= platform.y + platform.height / 2.0
}

fn player_killed(player: &Player, enemy: &Enemy) -> bool {
    (player.texture.collide_left(&enemy.texture) || player.texture.collide_right(&enemy.texture))
        && !enemy.dead
}

pub struct Topbar {
    pub fish_score: Score,
    pub fish_label: Texture,
    pub enemy_score: Score,
    pub enemy_label: Texture,
    pub flag_label: Texture,
    pub distance: Score,
    pub stop_button: Texture,
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

pub struct Game {
    player: Player,
    platforms: Vec<Platform>,
    enemies: Vec<Enemy>,
    controls: Vec<Texture>,
    elapsed_time: f32,
    spawn_time: f32,
    game_over_delay: f32,
    rand: ThreadRng,
    topbar: Topbar,
    paused: bool,
}

impl Game {
    pub fn new(display: &Display) -> Self {
        let p = Player::new(display);

        let mut platforms: Vec<Platform> = vec![];

        let mut starting_platform = Platform::new(display, Size::XLarge);
        starting_platform.set_position(LEFT + 100.0, -50.0);
        platforms.push(starting_platform);

        for i in 0..3 {
            platforms.push(Platform::new(display, Size::from_u32(i % 3)));
        }

        platforms[3].set_position(510.0, -100.0);
        platforms[2].set_position(800.0, -150.0);
        platforms[1].set_position(1060.0, 50.0);

        let mut controls: Vec<Texture> = vec![];

        controls.push(Texture::new("./res/controls1.png", display));
        controls[0].scale(0.8);
        controls[0].set_position(-210.0, 160.0);

        controls.push(Texture::new("./res/controls2.png", display));
        controls[1].scale(0.8);
        controls[1].set_position(510.0, 160.0);

        Game {
            player: p,
            platforms: platforms,
            enemies: vec![],
            controls: controls,
            elapsed_time: 0.0,
            spawn_time: 0.0,
            game_over_delay: 0.0,
            rand: rand::thread_rng(),
            topbar: Topbar::new(display),
            paused: false,
        }
    }

    pub fn restart(&mut self, display: &Display) {
        self.game_over_delay = 0.0;

        self.platforms[3].set_position(510.0, -100.0);
        self.platforms[2].set_position(800.0, -150.0);
        self.platforms[1].set_position(1060.0, 50.0);
        self.platforms[0].set_position(LEFT + 100.0, -50.0);

        for i in 0..self.platforms.len() {
            self.platforms[i].set_type(Type::Plain);
        }

        self.controls[0].set_position(-210.0, 160.0);
        self.controls[1].set_position(510.0, 160.0);

        self.enemies.clear();
        self.player.reset();
        self.topbar.reset(display);
    }

    pub fn game_over(&mut self, dt: f32) -> bool {
        if self.player.dead {
            self.game_over_delay += dt;
            if self.game_over_delay > 3.0 {
                return true;
            }
        }
        false
    }

    pub fn paused(&self) -> bool {
        self.paused
    }

    pub fn resume(&mut self) {
        self.paused = false;
    }

    pub fn update(&mut self, input: &mut InputManager, display: &Display, dt: f32) {
        if input.key_went_up(VirtualKeyCode::Escape) {
            self.paused = true;
        }

        if self.paused {
            return;
        }

        self.player.update(input, dt);

        for i in 0..self.platforms.len() {
            self.platforms[i].update(display, dt);
            match self.platforms[i].platform_type {
                Type::Fish => {
                    for j in 0..self.platforms[i].fish.len() {
                        if intersect(&self.platforms[i].fish[j].texture, &self.player.texture)
                            && !self.platforms[i].fish[j].taken
                        {
                            self.topbar.fish_score.increment(display);
                            self.platforms[i].fish[j].taken = true;
                        }
                    }
                }
                Type::Enemy => {
                    for j in 0..self.platforms[i].enemies.len() {
                        if self
                            .player
                            .texture
                            .collide_bottom(&self.platforms[i].enemies[j].texture)
                            && !self.player.dead
                            && !self.platforms[i].enemies[j].dead
                        {
                            self.topbar.enemy_score.increment(display);
                            self.platforms[i].enemies[j].dead = true;
                        } else if player_killed(&self.player, &self.platforms[i].enemies[j]) {
                            self.player.dead = true;
                        }
                    }
                }
                _ => {}
            }
        }

        for i in 0..self.enemies.len() {
            self.enemies[i].update(dt);
            if !self.enemies[i].dead {
                self.enemies[i].translate(-120.0 * dt, 0.0);
            } else {
                self.enemies[i].apply_gravity(dt);
            }
            if self.enemies[i].x <= LEFT - self.enemies[i].width
                || self.enemies[i].y <= BOTTOM - self.enemies[i].height
            {
                let x = self.rand.gen_range(RIGHT..SCREEN_WIDTH);
                let y = self.rand.gen_range(BOTTOM + 40.0..TOP - 40.0);
                self.enemies[i].set_position(x, y);
                self.enemies[i].dead = false;
            }
            if self.player.texture.collide_bottom(&self.enemies[i].texture)
                && !self.player.dead
                && !self.enemies[i].dead
            {
                self.topbar.enemy_score.increment(display);
                self.enemies[i].dead = true;
            } else if player_killed(&self.player, &self.enemies[i]) {
                self.player.dead = true;
            }
        }

        for i in 0..self.platforms.len() {
            if self.platforms[i].x + self.platforms[i].width / 2.0 < (-SCREEN_WIDTH) {
                let mut x: f32;
                let mut y: f32;
                let w = self.platforms[i].width;
                let h = self.platforms[i].height;
                loop {
                    let mut intersects = false;
                    y = self.rand.gen_range(BOTTOM + 100.0..TOP - 200.0);
                    x = self.rand.gen_range(RIGHT + 100.0..SCREEN_WIDTH + RIGHT);
                    for j in 0..self.platforms.len() {
                        if overlap_x(
                            Rect {
                                x: x,
                                y: y,
                                w: w,
                                h: h,
                            },
                            Rect {
                                x: self.platforms[j].x,
                                y: self.platforms[j].y,
                                w: self.platforms[j].width,
                                h: self.platforms[j].height,
                            },
                        ) && i != j
                        {
                            intersects = true;
                            break;
                        }
                    }
                    if !intersects {
                        break;
                    }
                }
                let p = self.rand.gen_range(0..10);
                if p < 5 {
                    self.platforms[i].set_type(Type::Fish);
                } else if p >= 5 && p < 8 {
                    self.platforms[i].set_type(Type::Enemy);
                } else {
                    self.platforms[i].set_type(Type::Plain);
                }
                self.platforms[i].set_position(x, y);
            }
        }

        for i in 0..self.platforms.len() {
            if player_landed(&self.player, &self.platforms[i]) {
                self.player.velocity[1] = 0.0;
            }

            if overlap_x(
                Rect {
                    x: self.player.x,
                    y: self.player.y,
                    w: self.player.width,
                    h: self.player.height,
                },
                Rect {
                    x: self.platforms[i].x,
                    y: self.platforms[i].y,
                    w: self.platforms[i].width,
                    h: self.platforms[i].height,
                },
            ) {
                self.player.on_platform = true;
                break;
            } else {
                self.player.on_platform = false;
            }
        }

        if self.elapsed_time > 999999. {
            self.elapsed_time = 1.0;
        }
        self.elapsed_time += dt;

        for i in 0..self.controls.len() {
            let t = self.elapsed_time * 1.5;
            let y = t.sin() * 0.04;

            self.controls[i].translate(0.0, y);
        }

        if self.player.right {
            for i in 0..self.platforms.len() {
                self.platforms[i].translate(-80.0 * dt, 0.0);
            }
            for i in 0..self.controls.len() {
                self.controls[i].translate(-80.0 * dt, 0.0);
            }
        }

        if self.player.distance > 0.5 {
            self.topbar.distance.increment(display);
            self.player.distance = 0.0;
        }

        self.spawn_time += dt;

        if self.spawn_time >= SPAWN_DELAY {
            self.enemies.push(Enemy::new(display, Species::Flying));
            let x = self.rand.gen_range(RIGHT..SCREEN_WIDTH);
            let y = self.rand.gen_range(BOTTOM..TOP);
            let i = self.enemies.len() - 1;
            self.enemies[i].set_position(x, y);
            self.spawn_time = 0.0;
        }
    }

    pub fn draw(&mut self, target: &mut Frame, program: &Program) {
        for i in 0..self.controls.len() {
            self.controls[i].draw(target, program);
        }
        for i in (0..=self.platforms.len() - 1).rev() {
            self.platforms[i].draw(target, program);
        }

        self.player.draw(target, program);

        for i in 0..self.enemies.len() {
            self.enemies[i].draw(target, program);
        }

        self.topbar.draw(target, program);
    }
}
