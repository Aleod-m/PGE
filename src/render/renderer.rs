use super::Camera;
use super::{VArray, IndexBuffer, ShaderProgram};

pub struct Renderer {
    cam : Camera,
    
}

impl Renderer {

    pub fn new(cam : Camera) -> Self{
        Self {
            cam,
        }
    }

    pub fn submit(vertex_array : VArray, shaders : ShaderProgram) {
        
    }

    pub fn submit_indexed(vertex_array : VArray, index_buffer : IndexBuffer, shaders : ShaderProgram ) {

    }

    pub fn draw(&self) {

    }
}
