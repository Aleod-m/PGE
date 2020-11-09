// Extern imports
use std::mem;
use gl::types::*;
// Crate imports
use crate::math::{Vec3D, Vec2D};
use super::GlObj;

pub struct Vertex {
    vertice : Vec3D,
    color : Vec3D,
    uv : Vec2D,
}

impl Vertex {
    pub fn to_data(&self) -> Vec<f64> {
        let mut v : Vec<f64> = Vec::with_capacity(8);
        v.extend(self.vertice.to_vec());
        v.extend(self.color.to_vec());
        v.extend(self.uv.to_vec());
        return v;
    }
}


pub struct Vbuf {
    _id : GLuint,
    verticies : Vec<f64>,
}

impl Vbuf {
    pub fn new(data: Vec<Vertex>) -> Self {
        unsafe {
            let verticies = data.iter().map(|vertex| vertex.to_data());
            let verticies = {
                let mut v = Vec::<f64>::new();
                for vert in verticies {
                    v.extend(vert.iter())
                }
                v
            };
            let mut buf = Self {
                _id : mem::zeroed(),
                verticies : verticies,
            };
            gl::GenBuffers(1 as GLsizei, &mut buf._id);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (data.len() * mem::size_of::<f64>()) as GLsizeiptr,
                data.as_ptr() as *const GLvoid,
                gl::STATIC_DRAW);
            buf
        }
    }

}

impl GlObj for Vbuf {

    fn id(&self) -> GLuint{
        self._id
    }

    fn bind(&self) {
        unsafe {gl::BindBuffer(gl::ARRAY_BUFFER, self._id)};
    }

    fn unbind(&self) {
        unsafe {gl::BindBuffer(gl::ARRAY_BUFFER, 0 as GLuint)};
    }
}
