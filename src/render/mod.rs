use gl::types::*;
pub mod ibuf;
pub mod vbuf;
pub mod shader;
pub mod varray;
pub mod texture;
pub use ibuf::Ibuf;
pub use vbuf::Vbuf;


pub trait GlObj {
    fn id(&self) -> GLuint;
    fn bind(&self) -> ();
    fn unbind(&self) -> ();
}