// External imports
use std::io::{Read, BufReader, Seek};
use gl::types::*;
use image::{
    load,
    RgbImage,
    Rgba,
    io::Reader as ImageReader
};
// Crate imports
use crate::utils::color::rgb::RgbColor;
use crate::ressources::{self, Ressources};
use super::{GlObj, TextureType};

/// Generic struct for 2D textures
pub(crate) struct Texture<T> 
where T : TextureType {
    _id : GLuint,
    gl : gl::Gl,
    data : Vec<u8>,
    width : usize,
    height : usize,
    _marker : std::marker::PhantomData<T>,
}


impl<T> Texture<T>
where T : TextureType {

    pub fn from_res(gl : gl::Gl, res : &Ressources, name : &str) -> Result<Self, Error> {
        let file = res.get_file(name)?;
        let img = ImageReader::new(BufReader::new(file)).with_guessed_format().unwrap().decode().unwrap().to_rgba8();
        
        Ok(Self::new_blank(gl, 10, 10))
    }

    pub fn from_data(gl : gl::Gl, data : Vec<u8>, width : usize, height : usize) -> Result<Self, Error> {
        if data.len() != width * height * T::format.get_pixel_size() {
            return Err(Error::SizeMismatch);
        }
        let mut _id = 0;
        unsafe { gl.GenTextures(1, &mut _id); }
        Ok(Self {
            _id,
            gl : gl.clone(),
            data,
            width,
            height,
            _marker : std::marker::PhantomData,
        })
    }


    pub fn from_color(gl : gl::Gl, width : usize, height : usize, color : RgbColor) -> Self {
        let data  = [color.red, color.green, color.blue, color.alpha.unwrap_or(255)]
            .iter()
            .cycle()
            .take(width * height * 4)
            .map(|e| *e)
            .collect();
        Self {
            _id : 0,
            gl : gl.clone(),
            data,
            width,
            height,
            _marker : std::marker::PhantomData,
        }
    }

    pub fn new_blank(gl : gl::Gl, width : usize, height : usize) -> Self {
        Self {
            _id : 0,
            gl : gl.clone(),
            data : Vec::with_capacity(width * height * 4),
            width,
            height,
            _marker : std::marker::PhantomData, 
        }
    }

    pub fn set_pixel(&mut self, i : usize, j : usize, color : RgbColor) {
        self.data[(i * self.width + j) * 3] = color.red;
        self.data[(i * self.width + j) * 3 + 1] = color.blue;
        self.data[(i * self.width + j) * 3 + 2] = color.green;
    }

    pub fn set_data(&self) {
        self.bind();
        unsafe { 
            self.gl.TexImage2D(
                gl::TEXTURE_2D,
                0,
                T::format.get_gl_type() as i32,
                self.width as i32,
                self.height as i32,
                0, T::format.get_gl_type(),
                gl::UNSIGNED_BYTE,
                self.data.as_ptr() as *const GLvoid
            )
        }
    }
}


impl<T> GlObj for Texture<T> 
where T : TextureType {
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

pub enum Error {
    SizeMismatch,
    Res(ressources::Error),
}

impl From<ressources::Error> for Error {
    fn from(other: ressources::Error) -> Self {
        Error::Res(other)
    }
}