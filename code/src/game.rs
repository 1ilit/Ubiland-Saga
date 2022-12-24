use glium::{Display, Program, Frame};

use crate::texture::Texture;

pub struct Game{
    texture: Texture,
}

impl Game {
    pub fn new(display: &Display)->Self{
        let mut tex=Texture::new(
            "C:\\Users\\Lilit\\Desktop\\ubiland\\code\\res\\rect.png",
            display,
        );
        tex.clip(0.0, 0.0, 32.0, 32.0);
        Game { texture: tex }
    }

    pub fn update(&mut self){}

    pub fn draw(&mut self, target: &mut Frame, program: &Program){
        self.texture.draw(target, program);
    }
}
