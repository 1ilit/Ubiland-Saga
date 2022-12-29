use std::fs::read;
use std::io::Cursor;

use glium::{uniform, Surface};

use crate::shape::Rectangle;

struct Rect {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
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
                x: 0.0,
                y: 0.0,
                width: 1.0,
                height: 1.0,
            },
            rect: rect,
        }
    }

    /**
     * follows tex coords
     * 0, 1         1, 1
     *  |
     *  |
     * 0, 0 _______ 1, 0
     */
    pub fn clip(&mut self, x: f32, y: f32, w: f32, h: f32) {
        let x0 = x / self.width;
        let y0 = y / self.height;
        let w0 = w / self.width;
        let h0 = h / self.height;

        println!("{}, {}, {}, {}", x0, y0, w0, h0);

        self.clipped = true;
        self.clip_rect = Rect {
            x: x0,
            y: y0,
            width: w0,
            height: h0,
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
            c_x: self.clip_rect.x,
            c_y: self.clip_rect.y,
            c_w: self.clip_rect.width,
            c_h: self.clip_rect.height,
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
