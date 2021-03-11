// External imports
use gl::{self, LINEAR, TEXTURE_MIN_FILTER, UNSIGNED_BYTE, types::*};
use image::io::Reader as ImageReader;
// Crate imports
use crate::utils::color::rgb::RgbColor;
use crate::ressources::{self, RessourceLoader};

pub enum Error {
    ResourceLoad { name: String, inner: ressources::Error },
    SizeMismatch,
}

pub struct Texture {
    id : GLuint,
    gl : gl::Gl,
    data : Vec<u8>,
    width : i32,
    height : i32,
}


// all the file format supported by the image crate
const POSSIBLE_EXT: [&str; 6] = [
            ".png",
            ".jpg",
            ".gif",
            ".ico",
            ".bmp",
            ".tiff",
        ];

impl Texture {

    pub fn from_res(gl : &gl::Gl, res : &RessourceLoader, name : &str) -> Self {
        let ressources_names = POSSIBLE_EXT
            .iter()
            .map(|(file_ext)| format!("{}{}", name, file_ext));
        // filter out the ones that don't exists
        //.partition(|name| res.name_to_path(&(**name).to_owned()).exists())
        //.0;
        

        let image = ImageReader::open(res.name_to_path(name))
            .unwrap()
            .decode()
            .unwrap()
            .to_rgba8();
        let width = image.width() as i32;
        let height = image.height() as i32;
        let mut id = 0;
        let data = image.into_raw();
        unsafe {
            gl.CreateTextures(gl::TEXTURE_2D, 1, &mut id);
            gl.TextureStorage2D(id, 1, gl::RGB8, width, height);
            gl.TextureParameteri(id, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl.TextureParameteri(id, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl.TextureSubImage2D(id, 0, 0, 0, width, height, gl::RGB, gl::UNSIGNED_BYTE, data.as_ptr() as *const GLvoid)
        }

        Self {
            id,
            gl : gl.clone(),
            data,
            width,
            height,

        }
    }

    pub fn set_pixel(&mut self, i : usize, j : usize, color : RgbColor) {
        self.data[(i * self.width as usize + j) * 3] = color.red;
        self.data[(i * self.width as usize + j) * 3 + 1] = color.blue;
        self.data[(i * self.width as usize + j) * 3 + 2] = color.green;
    }


    pub fn id(&self) -> GLuint {
        self.id
    }

    pub fn bind(&self, slot : GLuint) {
        unsafe {self.gl.BindTextureUnit(slot, self.id)};
    }

}