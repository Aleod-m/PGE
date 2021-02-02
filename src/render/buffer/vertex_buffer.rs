use super::{Buffer, BufferType};
use super::super::GlObj;
use gl::{self, types::*};

struct VertexBufferType;
impl BufferType for VertexBufferType {
    const TYPE : GLuint = gl::ARRAY_BUFFER;
}

pub struct VertexBuffer {
    inner : Buffer<VertexBufferType>,
}

impl VertexBuffer {
    pub fn new(gl : &gl::Gl) -> Self {
        Self{inner : Buffer::<VertexBufferType>::new(gl)}
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