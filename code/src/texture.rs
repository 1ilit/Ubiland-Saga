extern crate glium;
extern crate image;
use std::io::Cursor;
use std::fs::read;
use glium::*;

const SCREEN_HEIGHT: f32 = 600.0;
const SCREEN_WIDTH: f32 = 720.0;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    color: [f32; 4],
    tex_coords: [f32; 2],
}

implement_vertex!(Vertex, position, color, tex_coords);

pub struct Texture {
    pub width: u32,
    pub height: u32,

    texture: glium::texture::SrgbTexture2d,
    vertex_buffer: glium::VertexBuffer<Vertex>,
    index_buffer: glium::index::NoIndices,
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

        let x=((image_dimensions.0 as f32)*2./SCREEN_WIDTH)/2.;
        let y=((image_dimensions.1 as f32)*2./SCREEN_HEIGHT)/2.;

        println!("x: {}, y: {}", x, y);

        let vertex1 = Vertex { //btm right
            position: [x, -y],
            color: [0.0, 0.0, 1.0, 1.0],
            tex_coords: [1.0, 0.0],
        };
        let vertex2 = Vertex { // top right
            position: [x, y],
            color: [0.0, 1.0, 0.0, 1.0],
            tex_coords: [1.0, 1.0]
        };
        let vertex3 = Vertex { //btm left
            position: [-x, -y],
            color: [1.0, 0.0, 1.0, 1.0],
            tex_coords: [0.0, 0.0]
        };
        let vertex4 = Vertex { //top left
            position: [-x, y],
            color: [0.0, 0.0, 1.0, 1.0],
            tex_coords: [0.0, 1.0]
        };

        let shape= vec![vertex1, vertex2, vertex3, vertex4];

        Self{
            width: image_dimensions.0,
            height: image_dimensions.1,
            texture: texture,
            vertex_buffer: glium::VertexBuffer::new(display, &shape).unwrap(),
            index_buffer: glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip)
        }
    }

    pub fn draw(&self, target: &mut glium::Frame, program: &glium::Program){
        let uniforms = uniform! {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0f32],
            ],
            isTex: true,
            tex: &self.texture,
        };
        target.draw(
                &self.vertex_buffer,
                &self.index_buffer,
                program,
                &uniforms,
                &Default::default(),
            )
            .unwrap();
    }
}
