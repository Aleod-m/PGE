use gl::types::*;
use super::GlObj;


pub struct Ibuf {
    _id : GLuint
}

impl Ibuf {
    pub fn new(vertices : Vec<u32>) -> Self {
        let buffer = unsafe {
            let mut buf = Self {
                _id : std::mem::zeroed(),
            };
            gl::GenBuffers(1 as GLsizei, &mut buf._id);
            buf.bind();
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * std::mem::size_of::<u32>()) as GLsizeiptr,
                vertices.as_ptr() as GLeglImageOES,
                gl::STATIC_DRAW);
            buf.unbind();
            buf
        };
        buffer
    }

}

impl GlObj for Ibuf {
    fn id(&self) -> GLuint {
        self._id
    }

    fn bind(&self) {
        unsafe {gl::BindBuffer(gl::ARRAY_BUFFER, self._id)};
    }

    fn unbind(&self) {
        unsafe {gl::BindBuffer(gl::ARRAY_BUFFER, 0 as GLuint)};
    }
}

impl Drop for Ibuf {
    fn drop(&mut self){
        unsafe {gl::DeleteBuffers(1 as GLsizei, &self._id)};
    }
}
