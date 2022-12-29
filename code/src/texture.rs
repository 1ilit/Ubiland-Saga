use std::fs::read;
use std::io::Cursor;

use glium::{uniform, Surface};

use crate::shape::Rectangle;

struct Rect {
    start: [f32; 2],
    size: [f32; 2],
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
    pub fn new(path: &str, display: &glium::Display) -> Self {
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

    pub fn clip(&mut self, x: f32, y: f32, w: f32, h: f32) {
        self.clipped = true;
        self.clip_rect = Rect {
            start: [x, y],
            size: [w, h],
        };
    }

    pub fn scale(&mut self, factor: f32) {
        self.rect.scale(factor);

        self.height *= factor;
        self.width *= factor;
    }

    pub fn translate(&mut self, x: f32, y: f32) {
        self.rect.translate(x, y);
    }

    pub fn set_position(&mut self, x: f32, y: f32) {
        self.rect.set_position(x, y);
    }

    pub fn draw(&self, target: &mut glium::Frame, program: &glium::Program) {
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
