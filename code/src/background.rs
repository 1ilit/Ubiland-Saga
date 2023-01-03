use glium::{Display, Frame, Program};
use rand::rngs::ThreadRng;
use rand::Rng;

use crate::shape::{GradientDirection, Rectangle, SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::texture::Texture;

pub struct Background {
    background_clouds: [Texture; 3],
    rect: Rectangle,
    rand: ThreadRng,
}

impl Background {
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

        let mut rect = Rectangle::new(display, SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32);
        rect.set_color(display, [0.8, 0.5, 0.3, 1.0]);
        rect.set_gradient(
            display,
            [1.0, 0.45, 1.0, 0.8],
            [0.3, 0.3, 1.0, 0.8],
            GradientDirection::Vertical,
        );

        Background {
            background_clouds: clouds,
            rect: rect,
            rand: rand::thread_rng(),
        }
    }

    pub fn update(&mut self) {
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
            self.background_clouds[i].translate(-0.015, 0.0);
        }
    }

    pub fn draw(&mut self, target: &mut Frame, program: &Program) {
        self.rect.draw(target, program);

        for i in 0..3 {
            self.background_clouds[i].draw(target, program);
        }
    }
}
