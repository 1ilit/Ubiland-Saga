use std::fs::read;
use std::io::Cursor;

use glium::{texture::SrgbTexture2d, uniform, Display, Surface};

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

    // pub fn sub_texture(&mut self, path: &str, display: &Display, x: f32, y: f32, w: f32, h: f32) -> Texture {
    //     let image = image::load(
    //         Cursor::new(read(path).expect("Unable to read file")),
    //         image::ImageFormat::Png,
    //     )
    //     .unwrap()
    //     .to_rgba8();

    //     let image_dimensions = image.dimensions();
    //     let image =
    //         glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);

    //     let texture = glium::texture::SrgbTexture2d::new(display, image).unwrap();

    //     let rect = Rectangle::new(display, image_dimensions.0, image_dimensions.1);
    //     Texture {
    //         width: w,
    //         height: h,
    //         texture: texture,
    //         clipped: true,
    //         clip_rect: Rect {
    //             start: [x, y],
    //             size: [w, h],
    //         },
    //         rect: rect,
    //     }
    // }

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

#[derive(Debug, PartialEq)]
pub enum AnimationMode {
    Loop,
    Once,
}

pub struct AnimatedTexture {
    pub width: f32,
    pub height: f32,

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
    pub fn new(
        display: &Display,
        paths: Vec<&str>,
        speed: f32,
        frames: usize,
    ) -> Self {
        let mut vec: Vec<Texture> = vec![];

        for i in 0..frames {
            vec.push(Texture::new(paths[i], display));
        }

        Self {
            width: vec[0].width,
            height: vec[0].height,
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

    pub fn _set_mode(&mut self, mode: AnimationMode) {
        self.mode = mode;
    }

    pub fn run_animation(&mut self, dt: f32) {
        self.animation_timer += dt;
        if self.animation_timer >= self.speed {
            if self.mode == AnimationMode::Loop {
                self.current_frame = (self.current_frame + 1) % self.frame_count as u8;
                self.animation_timer -= self.speed;
            } else {
                self.animation_done = true;
                self.animation_timer = self.speed - self.time_per_frame;
            }
        }
    }

    pub fn update(&mut self, dt: f32) {
        if !self.animation_done {
            self.run_animation(dt);
        }
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
            self.textures[i].rect.translate(x, y);
        }
    }

    fn get_position(&mut self) -> (f32, f32) {
        self.textures[0].rect.get_position()
    }

    fn set_position(&mut self, x: f32, y: f32) {
        for i in 0..self.frame_count {
            self.textures[i].rect.set_position(x, y);
        }
    }

    fn draw(&self, target: &mut glium::Frame, program: &glium::Program) {
        self.textures[self.current_frame as usize].draw(target, program);
    }
}
