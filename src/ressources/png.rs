// External imports
use std::convert::TryInto;
// Crate imports
use crate::ressources::{self, Ressources};


struct ChunkType {
    bytes : [u8; 4],
}

impl ChunkType {
    
    fn bytes(&self) -> [u8; 4] {
        self.bytes
    }
    //fn is_valid(&self) -> bool
    fn is_critical(&self) -> bool {
        self.bytes[0] & 32_u8 == 0
    }
    fn is_public(&self) -> bool {
        self.bytes[0] & 32_u8 == 0
    }
    fn is_reserved_bit_valid(&self) -> bool {
        self.bytes[0] & 32_u8 == 0
    }
    fn is_safe_to_copy(&self) -> bool {
        self.bytes[0] & 32_u8 == 1
    }
    fn as_string(&self) -> String {
        String::from_utf8(self.bytes.to_vec()).unwrap()
    }
}

impl From<[u8; 4]> for ChunkType {
    fn from(other : [u8; 4]) -> Self {
        Self {
            bytes : other
        }
    }
}


struct Chunk {
    length : u32,
    chunk_type : ChunkType,
    data : Vec<u8>,
    crc : [u8; 4],
}

impl Chunk {
    
    fn length(&self) -> u32 {
        self.length
    }
    fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }
    fn data(&self) -> &[u8] {
        &self.data[..]
    }

    fn crc(&self) -> u32 {
        let CRC_POL : u32 = 0xedb88320;
        let crc_table : [u32; 256] = [0; 256];
        let c : u32;
        let n = 0_u32;
        while n < 256 {
            n += 1_u32;
            c = n as u32;
            let k = 0_u8;
            while k < 8 {
                k += 1_u8;
                if c & 1_u32 == 1{
                    c = CRC_POL ^ (c >> 1);
                }
                else {
                    c = c>>1;
                }
            } 
            crc_table[n as usize] = c;
        }
        
        let buf : Vec<u8> = Vec::with_capacity(self.data.len() + 8);
        buf.extend_from_slice(&self.chunk_type.bytes);
        buf.extend_from_slice(self.data.as_slice());


        let c : u32 = 0xffffffff;
        let n = 0;
        while n < buf.len() {
            c = crc_table[((c ^ buf[n] as u32) & 0xff_u32 )as usize] ^ (c>>8);
        }
        return c ^ 0xffffffff;
    }

    fn as_bytes(&self) -> Vec<u8> {
        self.data //TODO complete in order to write in file
    }

}

#[derive(Debug)]
pub enum Error {
    ResourceLoad { name : String, inner : ressources::Error },
    PngFileCorrupted { name : String }
}



pub struct Png {
    chunks : Vec<Chunk>,
}



// The first 8 bytes in a png file
pub const PNG_SIGNATURE : [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];

impl Png {

    pub fn from_res(res : &Ressources, name: &str) -> Result<Png, Error> {
        const ext : &str = ".png";
        let chunks : Vec<Chunk> = Vec::new();

        let ressource_name = format!("{}{}", name, ext);

        let data = res.load_bytes(name)
        .map_err(|error| {Error::ResourceLoad{name : name.to_owned(), inner : error}})?;

        let signature : &[u8] = &data[0..8];
        if signature != &PNG_SIGNATURE[..] {
            return Err(Error::PngFileCorrupted{name : name.to_owned()});
        }

        let is_parse_complete = false;
        let chunk_indice = 8;
        while !is_parse_complete {
            let chunk : Chunk;
            let length = u32::from_be_bytes(data[chunk_indice..chunk_indice+4].try_into().unwrap());
            chunk_indice += 4;
            let chunk_type_bytes : [u8; 4] = data[chunk_indice..chunk_indice+4].try_into().unwrap();
            let chunk_type = ChunkType::from(chunk_type_bytes);
            chunk_indice += 4;
            let chunk_data = &data[chunk_indice..chunk_indice+length as usize];
            chunk_indice += length as usize;
            let crc : [u8; 4] = data[chunk_indice..chunk_indice+4].try_into().unwrap();
            chunk = Chunk {
                length,
                chunk_type,
                data : chunk_data.to_vec(),
                crc
            };
            chunks.push(chunk);
            if chunk_type.as_string() == "IEND" {
                is_parse_complete = true;
            }
        }

        Ok(Self {
            chunks,
        })
    }

}

