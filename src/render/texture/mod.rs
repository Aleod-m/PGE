mod texture;
use super::GlObj;
use gl::types::*;

pub enum TextureFormat {
    RGB,
    RGBA,
    GS,
    GSA,
}

impl TextureFormat {
    fn get_gl_type(&self) -> GLenum {
        match self {
            TextureFormat::GS => gl::R8UI,
            TextureFormat::GSA => gl::RG8UI,
            TextureFormat::RGB => gl::RGB8UI,
            TextureFormat::RGBA => gl::RGBA8UI,
        }
    }

    fn get_pixel_size(&self) -> usize {
        match self {
            TextureFormat::GS => 1,
            TextureFormat::GSA => 2,
            TextureFormat::RGB => 3,
            TextureFormat::RGBA => 4,
        }
    }
}


pub trait TextureType {
    const format : TextureFormat;
}

pub struct GSTextureType;
impl TextureType for GSTextureType {
    const format : TextureFormat = TextureFormat::GS;
} 
pub struct GSATextureType;
impl TextureType for GSATextureType {
    const format : TextureFormat = TextureFormat::GSA;
}
pub struct RGBTextureType;
impl TextureType for RGBTextureType {
    const format : TextureFormat = TextureFormat::RGB;
}

pub struct RGBATextureType;
impl TextureType for RGBATextureType {
    const format : TextureFormat = TextureFormat::RGBA;
}

