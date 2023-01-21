use glium::{glutin::event::VirtualKeyCode, Display, Frame, Program};
use rand::{rngs::ThreadRng, Rng};

use crate::{
    collision::overlap_x,
    enemy::{Enemy, Species, SPAWN_DELAY},
    input_mgr::InputManager,
    platform::{Platform, Size, Type},
    player::Player,
    shape::{BOTTOM, LEFT, RIGHT, SCREEN_WIDTH, TOP},
    texture::{Rect, Texture, Transform}, gui::Topbar,
};

pub struct Level {
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

impl Level {
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

        Level {
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
        if self.player.is_dead() {
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

    fn get_new_platform_pos(&mut self, index: usize) -> (f32, f32) {
        let mut x: f32;
        let mut y: f32;
        let w = self.platforms[index].width;
        let h = self.platforms[index].height;
        loop {
            let mut intersects = false;
            x = self.rand.gen_range(RIGHT + 100.0..SCREEN_WIDTH + RIGHT);
            y = self.rand.gen_range(BOTTOM + 100.0..TOP - 200.0);
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
                ) && index != j
                {
                    intersects = true;
                    break;
                }
            }
            if !intersects {
                break;
            }
        }
        (x, y)
    }

    fn regenerate_platform(&mut self, index: usize) {
        let p = self.rand.gen_range(0..10);
        if p < 5 {
            self.platforms[index].set_type(Type::Fish);
        } else if p >= 5 && p < 8 {
            self.platforms[index].set_type(Type::Enemy);
        } else {
            self.platforms[index].set_type(Type::Plain);
        }
        let (x, y) = self.get_new_platform_pos(index);
        self.platforms[index].set_position(x, y);
    }

    pub fn update(&mut self, input: &mut InputManager, display: &Display, dt: f32) {
        if self.paused {
            return;
        }

        if input.key_went_up(VirtualKeyCode::Escape) {
            self.paused = true;
        }

        self.player.update(input, dt);

        for i in 0..self.platforms.len() {
            self.platforms[i].update(display, dt);

            match self.platforms[i].platform_type {
                Type::Fish => {
                    self.platforms[i].player_took_fish(display, &self.player, &mut self.topbar);
                }
                Type::Enemy => {
                    self.platforms[i].player_vs_enemy(display, &mut self.player, &mut self.topbar);
                }
                _ => {}
            }

            if self.platforms[i].x + self.platforms[i].width / 2.0 < (-SCREEN_WIDTH) {
                self.regenerate_platform(i);
            }

            if player_landed(&self.player, &self.platforms[i]) {
                self.player.velocity[1] = 0.0;
            }

            if self.platforms[i].player_is_on(&mut self.player) {
                break;
            }
        }

        for i in 0..self.enemies.len() {
            self.enemies[i].update(dt);

            if !self.enemies[i].is_dead() {
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
                self.enemies[i].set_dead(false);
            }

            self.player.check_interaction(&mut self.enemies[i], &mut self.topbar, display);
        }

        if self.controls[1].x > -SCREEN_WIDTH {
            if self.elapsed_time > 999999. {
                self.elapsed_time = 1.0;
            }
            self.elapsed_time += dt;

            for i in 0..self.controls.len() {
                let t = self.elapsed_time * 1.5;
                let y = t.sin() * 0.04;

                self.controls[i].translate(0.0, y);
            }
        }

        if self.player.is_moving_right {
            for i in 0..self.platforms.len() {
                self.platforms[i].translate(-80.0 * dt, 0.0);
            }
            for i in 0..self.controls.len() {
                self.controls[i].translate(-80.0 * dt, 0.0);
            }
        }

        if self.player.distance > 0.5 {
            self.topbar.increment_distance(display);
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

fn player_landed(player: &Player, platform: &Platform) -> bool {
    player.x + player.width / 2. >= platform.x - platform.width / 2.
        && player.x - player.width / 2. <= platform.x + platform.width / 2.
        && player.y - player.height / 2. + player.velocity[1] <= platform.y + platform.height / 2.
        && player.y - player.height / 2. >= platform.y + platform.height / 2.0
}
