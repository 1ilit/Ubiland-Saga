use std::fs::read;
use std::io::Cursor;

use glium::{uniform, Display, Surface};

use crate::shape::{Direction, Rectangle};

struct Rect {
    start: [f32; 2],
    size: [f32; 2],
}

pub trait Transform {
    fn scale(&mut self, factor: f32);
    fn translate(&mut self, x: f32, y: f32);
    fn set_position(&mut self, x: f32, y: f32);
    fn get_position(&mut self) -> (f32, f32);
    fn draw(&self, target: &mut glium::Frame, program: &glium::Program);
}

pub struct Texture {
    pub width: f32,
    pub height: f32,

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

        let rect = Rectangle::new(display, image_dimensions.0, image_dimensions.1);

        Self {
            width: image_dimensions.0 as f32,
            height: image_dimensions.1 as f32,
            texture: texture,
            clipped: false,
            clip_rect: Rect {
                start: [0.0, 0.0],
                size: [1.0, 1.0],
            },
            rect: rect,
        }
    }

    pub fn _clip(&mut self, x: f32, y: f32, w: f32, h: f32) {
        self.clipped = true;
        self.clip_rect = Rect {
            start: [x, y],
            size: [w, h],
        };
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
    }

    fn get_position(&mut self) -> (f32, f32) {
        self.rect.get_position()
    }

    fn set_position(&mut self, x: f32, y: f32) {
        self.rect.set_position(x, y);
    }

    fn draw(&self, target: &mut glium::Frame, program: &glium::Program) {
        let uniforms = uniform! {
            matrix: self.rect.matrix,
            isTex: true,
            tex: &self.texture,
            clipped: self.clipped,
            start: self.clip_rect.start,
            size: self.clip_rect.size,
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

#[derive(Debug, PartialEq)]
pub enum AnimationMode {
    Loop,
    Once,
}

pub struct AnimatedTexture {
    pub width: f32,
    pub height: f32,

    texture: glium::texture::SrgbTexture2d,
    clip_rect: Rect,
    rect: Rectangle,
    speed: f32,
    direction: Direction,
    animation_timer: f32,
    mode: AnimationMode,
    animation_done: bool,
    time_per_frame: f32,
    start_x: f32,
    start_y: f32,
}

impl AnimatedTexture {
    pub fn new(
        display: &Display,
        path: &str,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        speed: f32,
        frames: u32,
    ) -> Self {
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

        let rect = Rectangle::new(display, image_dimensions.0, image_dimensions.1);

        Self {
            width: w,
            height: h,
            texture: texture,
            clip_rect: Rect {
                start: [x, y],
                size: [w, h],
            },
            rect: rect,
            speed: speed,
            direction: Direction::Horizontal,
            animation_timer: 0.0,
            mode: AnimationMode::Loop,
            animation_done: false,
            time_per_frame: speed / frames as f32,
            start_x: x,
            start_y: y,
        }
    }

    pub fn _set_mode(&mut self, mode: AnimationMode) {
        self.mode = mode;
    }

    pub fn _set_direction(&mut self, dir: Direction) {
        self.direction = dir;
    }

    pub fn run_animation(&mut self, dt: f32) {
        self.animation_timer += dt;
        if self.animation_timer >= self.speed {
            if self.mode == AnimationMode::Loop {
                self.animation_timer -= self.speed;
            } else {
                self.animation_done = true;
                self.animation_timer = self.speed - self.time_per_frame;
            }
        }
        if self.direction == Direction::Horizontal {
            self.clip_rect.start[0] = (self.start_x as i32
                + (self.animation_timer / self.time_per_frame) as i32 * self.width as i32)
                as f32;
                
        } else {
            self.clip_rect.start[1] = (self.start_y as i32
                + (self.animation_timer / self.time_per_frame) as i32 * self.height as i32)
                as f32;
        }
        //  println!("{:?}", self.clip_rect.start);
    }

    pub fn update(&mut self, dt: f32) {
        if !self.animation_done {
            self.run_animation(dt);
        }
    }
}

impl Transform for AnimatedTexture {
    fn scale(&mut self, factor: f32) {
        self.rect.scale(factor);

        self.height *= factor;
        self.width *= factor;
    }

    fn translate(&mut self, x: f32, y: f32) {
        self.rect.translate(x, y);
    }

    fn get_position(&mut self) -> (f32, f32) {
        self.rect.get_position()
    }

    fn set_position(&mut self, x: f32, y: f32) {
        self.rect.set_position(x, y);
    }

    fn draw(&self, target: &mut glium::Frame, program: &glium::Program) {
        let uniforms = uniform! {
            matrix: self.rect.matrix,
            isTex: true,
            tex: &self.texture,
            clipped: true,
            start: self.clip_rect.start,
            size: self.clip_rect.size,
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
