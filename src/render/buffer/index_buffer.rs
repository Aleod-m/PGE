use super::{Buffer, BufferType};
use super::super::GlObj;
use gl::{self, types::*};


struct IndexBufferType;
impl BufferType for IndexBufferType {
    const TYPE : GLuint = gl::ELEMENT_ARRAY_BUFFER;
}
pub struct IndexBuffer {
    inner : Buffer<IndexBufferType>,
}

impl IndexBuffer {
    pub fn new(gl : &gl::Gl) -> Self {
        Self{inner : Buffer::<IndexBufferType>::new(gl)}
    }
    /// Set the buffer data on the GPU
    pub fn set_data<T>(&self, data : &[T]) {
        self.inner.set_data(data);
    }

    pub fn id(&self) -> GLuint{
        self.inner.id()
    }

    pub fn bind(&self) {
        self.inner.bind();
    }

    pub fn unbind(&self) {
        self.inner.unbind();
    }
}