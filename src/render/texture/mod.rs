mod texture;
pub mod pixel;
pub enum TextureFormat {
    RGB,
    RGBA,
    GS,
    GSA,
}

impl TextureFormat {
    fn get_pixel_size(&self) -> usize {
        match self {
            TextureFormat::GS => 1usize,
            TextureFormat::GSA => 2usize,
            TextureFormat::RGB => 3usize,
            TextureFormat::RGBA => 4usize,
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

