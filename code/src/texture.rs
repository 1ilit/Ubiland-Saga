extern crate glium;
extern crate image;
use std::io::Cursor;
use std::fs::read;

pub struct Texture {
    pub width: u32,
    pub height: u32,
    pub texture: glium::texture::SrgbTexture2d,
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

        Self{
            width: image_dimensions.0,
            height: image_dimensions.1,
            texture: texture,
        }
    }
}
