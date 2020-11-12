// External imports
use gl::types::*;
// Crate imports
use crate::math::Vec3D;
use crate::ressources::{self, Ressources};
use super::GlObj;

pub enum Error {
    ResourceLoad { name: String, inner: ressources::Error },
}

pub enum TextureFormat {
    RGB,
    RGBA,
    Grayscale,
    GrayscaleAlpha
}

pub struct Texture {
    _id : GLuint,
    gl : gl::Gl,
    data : Vec<u8>,
    width: u32,
    height: u32,
    format: TextureFormat
}

impl Texture {

    pub fn from_res(res : &Ressources, name : &str) /*-> Result<Texture, Error> */{
        //TODO fn from_res for Texture
    }

    pub fn from_data(data : Vec<u32>, format : TextureFormat) {
        //TODO fn from_data for Texture
    }


    pub fn from_color(w : u32, h : u32, color : Vec3D) {
        //TODO fn from_color for Texture
    }
}

impl GlObj for Texture {
    fn id(&self) -> GLuint {
        self._id
    }

    fn bind(&self) {
        unsafe {self.gl.BindTexture(gl::TEXTURE_2D, self._id)};
    }

    fn unbind(&self) {
        unsafe {self.gl.BindTexture(gl::TEXTURE_2D, 0)};
    }
}