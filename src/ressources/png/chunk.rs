use crate::ressources::png::chunktype::{self, ChunkType};

#[derive(Debug)]
pub enum Error {
    InvalidChunk(chunktype::Error),
}

pub struct Chunk {
    length : u32,
    chunk_type : ChunkType,
    data : Vec<u8>,
    crc : [u8; 4],
}

impl Chunk {
    
    pub fn length(&self) -> u32 { self.length }
    pub fn chunk_type(&self) -> &ChunkType { &self.chunk_type }
    pub fn data(&self) -> &[u8] { &self.data }

    fn crc_check(&self) -> bool {
        // x^32 + x^26 + x^23 + x^22 + x^16 + x^12 + x^11 + x^10 + x^8 + x^7 + x^5 + x^4 + x^2 + x + 1
        let crc_pol : u32 = 0xedb88320; 

        let mut crc_table : [u32; 256] = [0; 256];
        let mut c : u32;
        let mut n = 0_u32;
        while n < 256 {
            n += 1_u32;
            c = n as u32;
            let mut k = 0_u8;
            while k < 8 {
                k += 1_u8;
                if c & 1_u32 == 1{
                    c = crc_pol ^ (c >> 1);
                }
                else {
                    c = c >> 1;
                }
            } 
            crc_table[n as usize] = c;
        }
        
        let mut buf : Vec<u8> = Vec::with_capacity(self.data.len() + 8);
        buf.extend_from_slice(&self.chunk_type.bytes());
        buf.extend(self.data.iter());
        buf.extend_from_slice(&self.crc[..]);
        

        let mut c : u32 = 0xffffffff;
        let mut n = 0;
        while n < buf.len() {
            c = crc_table[((c ^ buf[n] as u32) & 0xff_u32 )as usize] ^ (c >> 8);
            n = n + 1;
        }
        return c ^ 0xffffffff == u32::from_be_bytes(self.crc);
    }

    fn as_bytes(&self) -> Vec<u8> {
        self.data.to_vec() //TODO complete in order to write in file
    }

}