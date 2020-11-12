// External imports
use std::convert::{TryInto, TryFrom};

#[derive(Debug)]
pub enum Error {
    InvalidChunk{message : String},
}

const CHUNK_TYPES : [[u8; 4]; 14] = [
    [b'I', b'H', b'D', b'R'], // IHDR Chunk
    [b'P', b'L', b'T', b'E'], // PLTE Chunk
    [b'I', b'D', b'A', b'T'], // IDAT Chunk
    [b'I', b'E', b'N', b'D'], // IEND Chunk
    [b'c', b'H', b'R', b'M'], // cHRM Chunk
    [b'g', b'A', b'M', b'A'], // gAMA Chunk
    [b's', b'B', b'I', b'T'], // sBIT Chunk
    [b'b', b'K', b'G', b'D'], // bKGD Chunk
    [b'h', b'I', b'S', b'T'], // hIST Chunk
    [b't', b'R', b'N', b'S'], // tRNS Chunk
    [b'p', b'H', b'Y', b's'], // pHYs Chunk
    [b't', b'I', b'M', b'E'], // tIME Chunk
    [b't', b'E', b'X', b't'], // tEXt Chunk
    [b'z', b'T', b'X', b't'], // zTXt Chunk
];

#[derive(Copy,Clone)]
pub struct ChunkType {
    bytes : [u8; 4],
}


impl ChunkType {
    
    // initialization from the bytes
    pub fn from_bytes(bytes : [u8; 4]) -> Result<Self, Error> {
        let c = Self{bytes};
        if c.is_valid() {
            Ok(c)
        }
        else {
            Err(Error::InvalidChunk {message : "The chunk is invalid!".to_string()})
        }
    }


    pub fn bytes(&self) -> [u8; 4] {
        self.bytes
    }

    // Chunk validity check
    pub fn is_critical(&self)           -> bool {self.bytes[0] & 32_u8 == 0}
    pub fn is_public(&self)             -> bool {self.bytes[1] & 32_u8 == 0}
    pub fn is_reserved_bit_valid(&self) -> bool {self.bytes[2] & 32_u8 == 0}
    pub fn is_safe_to_copy(&self)       -> bool {self.bytes[3] & 32_u8 == 1}
    pub fn is_valid(&self) -> bool {
        self.is_reserved_bit_valid()
    }
    
    pub fn as_string(&self) -> String {
        String::from_utf8(self.bytes().to_vec()).unwrap()
    }


}
