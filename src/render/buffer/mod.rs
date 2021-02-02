use gl::{self, types::*};
use std::mem::size_of;

use super::GlObj;
pub mod index_buffer;
pub mod vertex_buffer;
pub use index_buffer::IndexBuffer;
pub use vertex_buffer::VertexBuffer;

/// Generic Buffer struct for all OpenGL buffer types
trait BufferType {
    const TYPE : GLuint;
}

struct Buffer<B> 
where
    B : BufferType,
{
    gl : gl::Gl,
    id : GLuint,
    _marker : std::marker::PhantomData<B>,
}

impl<B> Buffer<B>
where
    B : BufferType
{
    /// Creates a new buffer 
    pub fn new(gl : &gl::Gl) -> Self {
        let mut id : GLuint = 0;
        unsafe{gl.GenBuffers(1, &mut id)}
        Self {
            gl : gl.clone(),
            id,
            _marker : std::marker::PhantomData,
        }
    }

    /// Set the buffer data on the GPU
    pub fn set_data<T>(&self, data : &[T]) {
        self.bind();
        unsafe{
            self.gl.BufferData(
                B::TYPE,
                (data.len() * size_of::<T>()) as GLsizeiptr,
                data.as_ptr() as *const GLvoid,
                gl::STATIC_DRAW
            )
        }
    } 


}

impl<B> GlObj for Buffer<B> where B : BufferType {

    fn id(&self) -> GLuint{
        self.id
    }
    fn bind(&self) {
        unsafe{self.gl.BindBuffer(B::TYPE, self.id)};
    }
    fn unbind(&self) {
        unsafe{self.gl.BindBuffer(B::TYPE, 0 as GLuint)};
    }
} 