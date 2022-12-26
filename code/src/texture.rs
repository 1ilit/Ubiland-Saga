extern crate glium;
extern crate image;
use glium::*;
use std::fs::read;
use std::io::Cursor;

const SCREEN_HEIGHT: f32 = 600.0;
const SCREEN_WIDTH: f32 = 720.0;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    color: [f32; 4],
    tex_coords: [f32; 2],
}

implement_vertex!(Vertex, position, color, tex_coords);

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
    vertex_buffer: glium::VertexBuffer<Vertex>,
    index_buffer: glium::index::NoIndices,
    clipped: bool,
    clip_rect: Rect,
    matrix: [[f32; 4]; 4],
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

        let x = ((image_dimensions.0 as f32) * 2. / SCREEN_WIDTH) / 2.;
        let y = ((image_dimensions.1 as f32) * 2. / SCREEN_HEIGHT) / 2.;

        println!("x: {}, y: {}", x, y);

        let vertex1 = Vertex {
            //btm right
            position: [x, -y],
            color: [0.0, 0.0, 1.0, 1.0],
            tex_coords: [1.0, 0.0],
        };
        let vertex2 = Vertex {
            // top right
            position: [x, y],
            color: [0.0, 1.0, 0.0, 1.0],
            tex_coords: [1.0, 1.0],
        };
        let vertex3 = Vertex {
            //btm left
            position: [-x, -y],
            color: [1.0, 0.0, 1.0, 1.0],
            tex_coords: [0.0, 0.0],
        };
        let vertex4 = Vertex {
            //top left
            position: [-x, y],
            color: [0.0, 0.0, 1.0, 1.0],
            tex_coords: [0.0, 1.0],
        };

        let shape = vec![vertex1, vertex2, vertex3, vertex4];

        Self {
            width: image_dimensions.0 as f32,
            height: image_dimensions.1 as f32,
            texture: texture,
            vertex_buffer: glium::VertexBuffer::new(display, &shape).unwrap(),
            index_buffer: glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip),
            clipped: false,
            clip_rect: Rect {
                x: 0.0,
                y: 0.0,
                width: 1.0,
                height: 1.0,
            },
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0f32],
            ],
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
        self.matrix[0][0] *= factor;
        self.matrix[1][1] *= factor;
        self.matrix[2][2] *= factor;

        self.height *= factor;
        self.width *= factor;
    }

    pub fn translate(&mut self, x: f32, y: f32) {
        let x0 = x * 2. / SCREEN_WIDTH;
        let y0 = y * 2. / SCREEN_HEIGHT;

        self.matrix[3][0] += x0;
        self.matrix[3][1] += y0;
    }

    pub fn set_position(&mut self, x: f32, y: f32) {
        let x0 = x * 2. / SCREEN_WIDTH;
        let y0 = y * 2. / SCREEN_HEIGHT;

        self.matrix[3][0] = x0;
        self.matrix[3][1] = y0;
    }

    pub fn draw(&self, target: &mut glium::Frame, program: &glium::Program) {
        let uniforms = uniform! {
            matrix: self.matrix,
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
                &self.vertex_buffer,
                &self.index_buffer,
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
