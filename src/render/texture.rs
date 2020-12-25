// External imports
use gl::types::*;
use image::io::reader;
// Crate imports
use crate::utils::color::rgb::RgbColor;
use crate::ressources::{self, Ressources};
use super::GlObj;

pub enum Error {
    ResourceLoad { name: String, inner: ressources::Error },
    SizeMismatch,
}

pub struct Texture {
    _id : GLuint,
    gl : gl::Gl,
    data : Vec<u8>,
    width : usize,
    height : usize,
}
const POSSIBLE_EXT: [&str; 6] = [
            ".png",
            ".jpg",
            ".gif",
            ".ico",
            ".bmp",
            ".tiff",
        ];

impl Texture {

    pub fn from_res(res : &Ressources, name : &str) {
        let ressources_names : Vec<String> = POSSIBLE_EXT.iter()
            // get all coresponding names
            .map(|(file_ext, _)| format!("{}{}", name, file_ext))
            // filter out the ones that don't exists
            .partition(|name| Ressources::name_to_path(&res.path.to_owned(), &(**name).to_owned()).exists()).0;
    }


    pub fn from_data(gl : gl::Gl, data : Vec<u8>, width : usize, height : usize) -> Result<Self, Error> {
        if data.len() != width * height * 4 { Err(Error::SizeMismatch) }
        Ok(Self {
            _id : 0,
            gl : gl.clone(),
            data,
            width,
            height,
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
        }
    }

    pub fn new_blank(gl : gl::Gl, width : usize, height : usize) -> Self {
        Self {
            _id : 0,
            gl : gl.clone(),
            data : Vec::with_capacity(width * height * 4),
            width,
            height,
        }
    }

    pub fn set_pixel(&self, i : usize, j : usize, color : RgbColor) {
        self.data[(i * self.width + j) * 3] = color.red;
        self.data[(i * self.width + j) * 3 + 1] = color.blue;
        self.data[(i * self.width + j) * 3 + 2] = color.green;
    }
    pub fn set_alpha() {

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