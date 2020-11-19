use gl::types::*;
pub mod ibuf;
pub mod vbuf;
pub mod shader;
pub mod varray;
pub mod texture;
pub mod buffer;
pub use buffer::*;
pub use ibuf::*;
pub use vbuf::*;
pub use shader::*;
pub use varray::*;
pub use texture::*;

pub trait GlObj {
    fn id(&self) -> GLuint;
    fn bind(&self) -> ();
    fn unbind(&self) -> ();
}