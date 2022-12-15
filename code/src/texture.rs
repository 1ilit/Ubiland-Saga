use gl;
use std;

pub struct Texture {
    id: gl::types::Glint,
}

pub impl Texture {
    pub fn new() -> Result<Texture, String> {
        let mut id: GLuint = 0;
        unsafe {
            gl::GenTextures(1, &mut id);
        }
        Ok(Texture{id});
    }

    pub unsafe fn bind(&self) {
        gl::BindTexture(gl::TEXTURE_2D, self.id)
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, self.id);
        }
    }
}
