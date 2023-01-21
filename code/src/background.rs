use glium::{Display, Frame, Program};
use rand::rngs::ThreadRng;
use rand::Rng;

use crate::shape::{Direction, Rectangle, BOTTOM, LEFT, RIGHT, SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::texture::{AnimatedTexture, Texture, Transform};

pub struct Background {
    background_clouds: [Texture; 3],
    birds: [AnimatedTexture; 5],
    background_gradient: Rectangle,
    rand: ThreadRng,
}

impl Background {
    pub fn new(display: &Display) -> Self {
        let mut big_cloud1 = Texture::new("./res/background/big_cloud.png", display);
        big_cloud1.set_position(0., BOTTOM + big_cloud1.height / 2.);

        let mut big_cloud2 = Texture::new("./res/background/big_cloud.png", display);
        big_cloud2.set_position(SCREEN_WIDTH - 3., BOTTOM + big_cloud2.height / 2.);

        let mut big_cloud3 = Texture::new("./res/background/rainbow.png", display);
        big_cloud3.set_position(SCREEN_WIDTH - 3., BOTTOM + big_cloud3.height / 2.);

        let clouds = [big_cloud1, big_cloud2, big_cloud3];

        let mut rect = Rectangle::new(display, SCREEN_WIDTH, SCREEN_HEIGHT);
        rect.set_color(display, [0.8, 0.5, 0.3, 1.0]);
        rect.set_gradient(
            display,
            [1.0, 0.45, 1.0, 0.8],
            [0.3, 0.3, 1.0, 0.8],
            Direction::Vertical,
        );

        let mut array = [
            AnimatedTexture::new(display, vec!["./res/background/bird1.png", "./res/background/bird2.png"], 0.3, 2),
            AnimatedTexture::new(display, vec!["./res/background/bird1.png", "./res/background/bird2.png"], 0.3, 2),
            AnimatedTexture::new(display, vec!["./res/background/bird1.png", "./res/background/bird2.png"], 0.3, 2),
            AnimatedTexture::new(display, vec!["./res/background/bird1.png", "./res/background/bird2.png"], 0.3, 2),
            AnimatedTexture::new(display, vec!["./res/background/bird1.png", "./res/background/bird2.png"], 0.3, 2),
        ];

        array[0].set_position(0.0, 0.0);
        array[1].set_position(100.0, -250.0);
        array[2].set_position(-300.0, -100.0);
        array[3].set_position(200.0, 170.0);
        array[4].set_position(-150.0, 100.0);

        Background {
            background_clouds: clouds,
            birds: array,
            background_gradient: rect,
            rand: rand::thread_rng(),
        }
    }

    pub fn update(&mut self, dt: f32) {
        if self.background_clouds[0].x + self.background_clouds[0].width / 2. < LEFT {
            self.background_clouds[0].set_x(SCREEN_WIDTH - 3.);
        }
        if self.background_clouds[1].x + self.background_clouds[1].width / 2. < LEFT {
            let x: u8 = self.rand.gen_range(0..4);
            if x == 3 {
                self.background_clouds[2].set_x(SCREEN_WIDTH - 3.);
            } else {
                self.background_clouds[1].set_x(SCREEN_WIDTH - 3.);
            }
        }

        for i in 0..3 {
            self.background_clouds[i].translate(-0.015, 0.0);
        }

        for i in 0..5 {
            self.birds[i].update(dt);
            self.birds[i].translate(0.012, 0.0);

            if self.birds[i].x >= RIGHT + 10. {
                self.birds[i].set_x(LEFT - 10.);
            }
        }
    }

    pub fn draw(&mut self, target: &mut Frame, program: &Program) {
        self.background_gradient.draw(target, program);

        for i in 0..3 {
            self.background_clouds[i].draw(target, program);
        }

        for i in 0..5 {
            self.birds[i].draw(target, program);
        }
    }
}
