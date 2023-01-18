use std::io::Cursor;
use std::{fs::read, vec};

use glium::{uniform, Display, Surface};

use crate::shape::{Direction, Rectangle, RIGHT, TOP};

pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

pub trait Transform {
    fn scale(&mut self, factor: f32);
    fn translate(&mut self, x: f32, y: f32);
    fn set_position(&mut self, x: f32, y: f32);
    fn set_x(&mut self, x: f32);
    fn set_y(&mut self, y: f32);
    fn mirror(&mut self, display: &Display, dir: Direction);
    fn get_position(&mut self) -> (f32, f32);
    fn clip(&mut self, x: f32, y: f32, w: f32, h: f32);
    fn draw(&self, target: &mut glium::Frame, program: &glium::Program);
}

pub trait Collide {
    fn collide_top(&self, other: &Self) -> bool;
    fn collide_bottom(&self, other: &Self) -> bool;
    fn collide_left(&self, other: &Self) -> bool;
    fn collide_right(&self, other: &Self) -> bool;
}

pub struct Texture {
    pub width: f32,
    pub height: f32,
    pub x: f32,
    pub y: f32,

    texture: glium::texture::SrgbTexture2d,
    clipped: bool,
    clip_rect: Rect,
    rect: Rectangle,
}

impl Texture {
    pub fn new(path: &str, display: &Display) -> Self {
        let image = image::load(
            Cursor::new(read(path).expect("Unable to read file")),
            image::ImageFormat::Png,
        )
        .unwrap()
        .to_rgba8();

        let image_dimensions = image.dimensions();
        let image =
            glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);

        let texture = glium::texture::SrgbTexture2d::new(display, image).unwrap();

        let rect = Rectangle::new(
            display,
            image_dimensions.0 as f32,
            image_dimensions.1 as f32,
        );

        Self {
            width: image_dimensions.0 as f32,
            height: image_dimensions.1 as f32,
            x: rect.matrix[3][0] * RIGHT,
            y: rect.matrix[3][1] * TOP,
            texture: texture,
            clipped: false,
            clip_rect: Rect {
                x: 0.0,
                y: 0.0,
                w: 1.0,
                h: 1.0,
            },
            rect: rect,
        }
    }

    pub fn get_dimensions(&self) -> (f32, f32) {
        (self.width, self.height)
    }
}

impl Transform for Texture {
    fn scale(&mut self, factor: f32) {
        self.rect.scale(factor);

        self.height *= factor;
        self.width *= factor;
    }

    fn translate(&mut self, x: f32, y: f32) {
        self.rect.translate(x, y);
        self.x = self.rect.matrix[3][0] * RIGHT;
        self.y = self.rect.matrix[3][1] * TOP;
    }

    fn get_position(&mut self) -> (f32, f32) {
        self.rect.get_position()
    }

    fn set_position(&mut self, x: f32, y: f32) {
        self.rect.set_position(x, y);
        self.x = self.rect.matrix[3][0] * RIGHT;
        self.y = self.rect.matrix[3][1] * TOP;
    }

    fn set_x(&mut self, x: f32) {
        self.rect.set_x(x);
        self.x = self.rect.matrix[3][0] * RIGHT;
    }

    fn set_y(&mut self, y: f32) {
        self.rect.set_y(y);
        self.y = self.rect.matrix[3][1] * TOP;
    }

    fn mirror(&mut self, display: &Display, dir: Direction) {
        self.rect.flip_tex_coords(display, dir);
    }

    fn clip(&mut self, x: f32, y: f32, w: f32, h: f32) {
        self.clipped = true;
        self.clip_rect = Rect {
            x: x,
            y: y,
            w: w,
            h: h,
        };
    }

    fn draw(&self, target: &mut glium::Frame, program: &glium::Program) {
        let uniforms = uniform! {
            matrix: self.rect.matrix,
            isTex: true,
            tex: &self.texture,
            clipped: self.clipped,
            start: [self.clip_rect.x, self.clip_rect.y],
            size: [self.clip_rect.w, self.clip_rect.h],
            anim: false,
        };
        target
            .draw(
                &self.rect.vertex_buffer,
                &self.rect.index_buffer,
                program,
                &uniforms,
                &glium::DrawParameters {
                    blend: glium::Blend::alpha_blending(),
                    ..Default::default()
                },
            )
            .unwrap();
    }
}

impl Collide for Texture {
    fn collide_bottom(&self, other: &Texture) -> bool {
        self.y - self.height / 2.0 >= other.y - other.height / 2.0
            && self.y - self.height / 2.0 <= other.y + other.height / 2.0
            && (self.x <= other.x + other.width / 2.0 && self.x >= other.x - other.width / 2.0)
    }

    fn collide_top(&self, other: &Texture) -> bool {
        other.collide_bottom(self)
    }

    fn collide_left(&self, other: &Texture) -> bool {
        (self.x - self.width / 2.0 >= other.x - other.width / 2.0
            && self.x - self.width / 2.0 <= other.x + other.width / 2.0)
            && (self.y <= other.y + other.height / 2.0 && self.y >= other.y - other.height / 2.0)
    }

    fn collide_right(&self, other: &Texture) -> bool {
        other.collide_left(self)
    }
}

#[derive(Debug, PartialEq)]
pub enum AnimationMode {
    Loop,
    Once,
}

pub struct AnimatedTexture {
    pub width: f32,
    pub height: f32,
    pub x: f32,
    pub y: f32,

    textures: Vec<Texture>,
    speed: f32,
    mode: AnimationMode,
    animation_timer: f32,
    time_per_frame: f32,
    animation_done: bool,
    current_frame: u8,
    frame_count: usize,
}

impl AnimatedTexture {
    pub fn new(display: &Display, paths: Vec<&str>, speed: f32, frames: usize) -> Self {
        let mut vec: Vec<Texture> = vec![];

        for i in 0..frames {
            vec.push(Texture::new(paths[i], display));
        }

        Self {
            width: vec[0].width,
            height: vec[0].height,
            x: vec[0].x,
            y: vec[0].y,
            textures: vec,
            speed: speed,
            animation_timer: 0.0,
            mode: AnimationMode::Loop,
            animation_done: false,
            time_per_frame: speed / frames as f32,
            current_frame: 0,
            frame_count: frames,
        }
    }

    pub fn set_mode(&mut self, mode: AnimationMode) {
        self.mode = mode;
    }

    pub fn run_animation(&mut self, dt: f32) {
        self.animation_timer += dt;
        if self.animation_timer >= self.speed {
            if self.mode == AnimationMode::Loop {
                self.current_frame = (self.current_frame + 1) % self.frame_count as u8;
                self.animation_timer -= self.speed;
            } else {
                self.current_frame += 1;
                if self.current_frame == self.frame_count as u8 {
                    self.animation_done = true;
                    self.current_frame -= 1;
                }
            }
        }
    }

    pub fn update(&mut self, dt: f32) {
        if !self.animation_done {
            self.run_animation(dt);
        }
    }

    pub fn get_dimensions(&self) -> (f32, f32) {
        (self.width, self.height)
    }
}

impl Transform for AnimatedTexture {
    fn scale(&mut self, factor: f32) {
        for i in 0..self.frame_count {
            self.textures[i].rect.scale(factor);
            self.textures[i].height *= factor;
            self.textures[i].width *= factor;
        }
    }

    fn translate(&mut self, x: f32, y: f32) {
        for i in 0..self.frame_count {
            self.textures[i].translate(x, y);
        }

        self.x = self.textures[0].x;
        self.y = self.textures[0].y;
    }

    fn get_position(&mut self) -> (f32, f32) {
        self.textures[0].get_position()
    }

    fn set_position(&mut self, x: f32, y: f32) {
        for i in 0..self.frame_count {
            self.textures[i].set_position(x, y);
        }

        self.x = self.textures[0].x;
        self.y = self.textures[0].y;
    }

    fn set_x(&mut self, x: f32) {
        for i in 0..self.frame_count {
            self.textures[i].set_x(x);
        }
        self.x = self.textures[0].x;
    }

    fn set_y(&mut self, y: f32) {
        for i in 0..self.frame_count {
            self.textures[i].set_y(y);
        }
        self.y = self.textures[0].y;
    }

    fn mirror(&mut self, display: &Display, dir: Direction) {
        for i in 0..self.frame_count {
            self.textures[i].mirror(display, dir);
        }
    }

    fn clip(&mut self, x: f32, y: f32, w: f32, h: f32) {
        for i in 0..self.frame_count {
            self.textures[i].clipped = true;
            self.textures[i].clip_rect = Rect {
                x: x,
                y: y,
                w: w,
                h: h,
            };
        }
    }

    fn draw(&self, target: &mut glium::Frame, program: &glium::Program) {
        self.textures[self.current_frame as usize].draw(target, program);
    }
}

impl Collide for AnimatedTexture {
    fn collide_bottom(&self, other: &AnimatedTexture) -> bool {
        self.y - self.height / 2.0 >= other.y - other.height / 2.0
            && self.y - self.height / 2.0 <= other.y + other.height / 2.0
            && (self.x <= other.x + other.width / 2.0 && self.x >= other.x - other.width / 2.0)
    }

    fn collide_top(&self, other: &AnimatedTexture) -> bool {
        other.collide_bottom(self)
    }

    fn collide_left(&self, other: &AnimatedTexture) -> bool {
        (self.x - self.width / 2.0 >= other.x - other.width / 2.0
            && self.x - self.width / 2.0 <= other.x + other.width / 2.0)
            && (self.y <= other.y + other.height / 2.0 && self.y >= other.y - other.height / 2.0)
    }

    fn collide_right(&self, other: &AnimatedTexture) -> bool {
        other.collide_left(self)
    }
}

pub struct Score {
    pub value: u32,
    textures: Vec<Texture>,
}

impl Score {
    pub fn new(display: &Display) -> Self {
        Self {
            value: 0,
            textures: vec![Texture::new("./res/digits/0.png", display)],
        }
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
        let x = self.textures[i-1].x;
        let y = self.textures[i-1].y;
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
