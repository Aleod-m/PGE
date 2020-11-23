// External imports
use gl::types::*;
// Crate imports
use crate::utils::color::rgb::RgbColor;
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
    width: usize,
    height: usize,
    format: TextureFormat,
}

impl Texture {

    pub fn from_res(res : &Ressources, name : &str) /*-> Result<Texture, Error> */{
        //TODO fn from_res for Texture
    }

    pub fn from_data(data : Vec<u32>, format : TextureFormat) {
        //TODO fn from_data for Texture
    }


    pub fn from_color(&self, gl : gl::Gl, width : usize, height : usize, color : RgbColor) -> Self {
        let data  = [color.red, color.green, color.blue].into_iter().cycle().take(width * height * 3).map(|e| *e).collect();
        Self {
            _id : 0,
            gl : gl.clone(),
            data,
            width,
            height,
            format : TextureFormat::RGB,
        }
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