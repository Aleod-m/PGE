use std;
use math::{
    Vec3D
};
use gl::types::*;

pub struct Vbuf {
    _id : GLuint
}

impl Vbuf {
    pub fn new(vertices : Vec<Vec3D>) -> Self {
        unsafe {
            let mut buf = Self {
                _id : std::mem::zeroed()
            };
            gl::GenBuffers(1 as GLsizei, &mut buf._id);
            buf.bind();
            let mut v : Vec<f64> = Vec::with_capacity(3 * vertices.len());
            for vx in vertices {
                v.push(vx.x);
                v.push(vx.y);
                v.push(vx.z);
            }
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (v.len() * std::mem::size_of::<f64>()) as GLsizeiptr,
                v.as_ptr() as *const GLvoid,
                gl::STATIC_DRAW);
            buf.unbind();
            buf
        }
    }

}
impl GlObj for Vbuf {

    pub fn id(&self) {
        self._id
    }

    pub fn bind(&self) {
        unsafe {gl::BindBuffer(gl::ARRAY_BUFFER, self._id)};
    }

    pub fn unbind(&self) {
        unsafe {gl::BindBuffer(gl::ARRAY_BUFFER, 0 as GLuint)};
    }
}
