use gl::types::*;
pub mod shader;
pub mod varray;
//pub mod texture;
pub mod buffer;
pub mod camera;

pub use buffer::*;
pub use shader::*;
pub use varray::*;
pub use camera::*;
//pub use texture::*;

pub trait GlObj {
    fn id(&self) -> GLuint;
    fn bind(&self) -> ();
    fn unbind(&self) -> ();
}