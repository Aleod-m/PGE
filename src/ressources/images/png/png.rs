// External imports
use std::convert::{TryInto, TryFrom};
// Crate imports
use crate::ressources::{self, Ressources};
use crate::render::texture::{Texture, TextureFormat};
use super::{
    chunktype::ChunkType,
    chunk::{self, Chunk},
};

struct Info
{
    pub width : u32,
    pub height : u32,
    pub color_type : u8,
    pub bit_depth : u8,
    pub compression_method : u8,
    pub filter_method : u8,
    pub interlace_method : u8,
    pub key_r : u32,
    pub key_g : u32,
    pub key_b : u32,
    pub key_defined : bool, //is a transparent color key given?
    pub palette : Vec<u8>,
}


#[derive(Debug)]
pub enum Error {
    PngFileCorrupted {name : String},
    InvalidChunkOrder{name : String, message : String},
    InvalidValue{name : String, message : String},
    ResourceLoad {name : String, inner : ressources::Error},
    InvalidChunk{name : String, inner : chunk::Error},
}



pub struct Png {
    chunks : Vec<Chunk>,
    name : String,
}

// The first 8 bytes in a png file
pub const PNG_SIGNATURE : [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];

impl Png {

    pub fn from_res(res : &Ressources, name: &str) -> Result<Png, Error> {
        const ext : &str = ".png";
        let mut chunks : Vec<Chunk> = Vec::new();

        let ressource_name = format!("{}{}", name, ext);

        let data = res.load_bytes(&ressource_name.to_owned())
        .map_err(|error| {Error::ResourceLoad{name : name.to_owned(), inner : error}})?;

        let signature : &[u8] = &data[0..8];
        if signature != &PNG_SIGNATURE[..] {
            return Err(Error::PngFileCorrupted{name : name.to_owned()});
        }

        let mut is_parse_complete = false;
        let mut chunk_indice = 8;
        while !is_parse_complete {
            let chunk : Chunk;
            let length = u32::from_be_bytes(data[chunk_indice..chunk_indice+4].try_into().unwrap());
            chunk_indice += 4;
            let chunk_type_bytes : [u8; 4] = data[chunk_indice..chunk_indice+4].try_into().unwrap();
            let chunk_type = ChunkType::from_bytes(chunk_type_bytes).unwrap();
            chunk_indice += 4;
            let chunk_data : Vec<u8> = data[chunk_indice..chunk_indice+length as usize].try_into().unwrap();
            chunk_indice += length as usize;
            let crc : [u8; 4] = data[chunk_indice..chunk_indice+4].try_into().unwrap();
            chunk = Chunk {
                length,
                chunk_type,
                data : chunk_data.to_vec(),
                crc,
            };
            chunks.push(chunk);
            if chunk_type.as_string() == "IEND" {
                is_parse_complete = true;
            }
        }

        
        Ok(Self {
            chunks,
            name : name.to_owned(),
        })
    }
    
    fn read_ihdr(&self) -> Result<Info, Error> {
        // Info struct containing all the information on the png file content
        let mut info = Info {
            width : 0,
            height : 0,
            color_type : 0,
            bit_depth : 0,
            compression_method : 0,
            filter_method : 0,
            interlace_method : 0,
            key_r : 0,
            key_g : 0,
            key_b : 0,
            key_defined : true, //is a transparent color key given?
            palette : Vec::<u8>::new(),
        };

        let ihdr = &self.chunks[0];
        if ihdr.chunk_type().as_string() == String::from("IHDR") {
            return Err(Error::InvalidChunkOrder{
                name : self.name.to_owned(),
                message : format!("The first chunk must be an IHDR chunk. Found {}", ihdr.chunk_type().as_string())
            });
        }
        info.width = u32::from_be_bytes(ihdr.data()[0..4].try_into().unwrap());
        info.height = u32::from_be_bytes(ihdr.data()[4..8].try_into().unwrap());
        info.color_type = ihdr.data()[9];
        info.bit_depth = ihdr.data()[8];
        info.compression_method = ihdr.data()[10];
        info.filter_method = ihdr.data()[11];
        info.interlace_method = ihdr.data()[12];
        Ok(info)
    }
    
    // pub fn as_texture(&self) -> Result<Texture, Error> {
    //     let texture : Texture;
    //     let mut info = self.read_ihdr().unwrap();
    //     info.key_defined = false;
    //     let mut chunks_end = false;
    //     while !chunks_end {
            
    //     }


    //     Ok(texture)
    // }

    fn deflate(data : Vec<u8>) {

    }
}

