use std;
use crate::math::{
    Vec3D,
    Vec2D
};
use super::GlObj;
use gl::types::*;

pub struct Vbuf {
    _id : GLuint,
    verticies_data : Vec<f64>,
}

impl Vbuf {
    pub fn new(data: Vec<f64>) -> Self {
        unsafe {
            let mut buf = Self {
                _id : std::mem::zeroed(),
                verticies_data : data,
            };
            gl::GenBuffers(1 as GLsizei, &mut buf._id);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (data.len() * std::mem::size_of::<f64>()) as GLsizeiptr,
                data.as_ptr() as *const GLvoid,
                gl::STATIC_DRAW);
            buf
        }
    }

    pub fn default() -> Self {
        let mut buf = Self {
            _id : std::mem::zeroed(),
            verts : None,
            colors : None,
            uvs : None,
        };
        gl::GenBuffers(1 as GLsizei, &mut buf._id);
        buf
    }


    fn update_data(&self){

        let mut it;
        let mut v : Vec<f64>;
        match (self.verts, self.colors, self.uvs) {

            (Some(verts), Some(colors), Some(uvs)) => {
                vlen += verts.len() * 8;
                it = verts.iter().zip(self.colors.iter()).zip(uvs.iter());
            }

            (Some(verts), None, Some(uvs)) => {
                vlen += verts.len() * 5;
                it = verts.iter().zip(uvs.iter());
            }

            (Some(verts), Some(colors), None) => {
                
            }
            (Some(verts), None, None) => {
                
            }

            (None, None, None) | _ => {}
        }

        let mut v : Vec<f64> = Vec::with_capacity(vlen);
        

    }
}

impl GlObj for Vbuf {

    fn id(&self) -> GLuint{
        self._id
    }

    fn bind(&self) {
        unsafe {gl::BindBuffer(gl::ARRAY_BUFFER, self._id)};
        self.update_data();
    }

    fn unbind(&self) {
        unsafe {gl::BindBuffer(gl::ARRAY_BUFFER, 0 as GLuint)};
    }
}
