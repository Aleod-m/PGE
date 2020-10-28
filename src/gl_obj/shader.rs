// External imports
use gl::types::*;
use std::path::{Path, PathBuf};
use std::ffi::{CString};
// Crate imports
use crate::ressources::{self, Ressources};
use super::GlObj;

#[derive(Debug)]
pub enum Error {
    ResourceLoad { name: String, inner: ressources::Error },
    CanNotDetermineShaderTypeForResource { name: String },
    CompileError { name: String, message: String },
    LinkError { name: String, message: String },
}

const POSSIBLE_EXT: [(&str, gl::types::GLenum); 6] = [
            (".vert",gl::VERTEX_SHADER),
            (".tesc",gl::TESS_CONTROL_SHADER),
            (".tese",gl::TESS_EVALUATION_SHADER),
            (".geom",gl::GEOMETRY_SHADER),
            (".frag",gl::FRAGMENT_SHADER),
            (".comp",gl::COMPUTE_SHADER),
        ];

fn create_empty_cstring_with_len(len: usize) -> CString {
    // allocate buffer of correct size
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    // fill it with len spaces
    buffer.extend([b' '].iter().cycle().take(len));
    // convert buffer to CString
    unsafe { CString::from_vec_unchecked(buffer) }
}

pub struct Program {
    _id : GLuint,
}

impl Program {

    pub fn from_res(res : &Ressources, name: &str) -> Result<Program, Error> {

        let ressources_names = POSSIBLE_EXT.iter()
        .map(|(file_ext, _)| format!("{}{}", name, file_ext))
        .collect::<Vec<String>>();

        let shaders = ressources_names.iter()
        .map(|ressource_name| Shader::from_res(res, ressource_name))
        .collect::<Result<Vec<Shader>, Error>>()?;

        Program::from_shaders(&shaders[..]).map_err(|message| Error::LinkError{name : name.to_owned(), message})

    }
    
    pub fn from_shaders(shaders : &[Shader]) -> Result<Program, String> {
        let id = unsafe { gl::CreateProgram()};

        for shader in shaders {
            unsafe { gl::AttachShader(id, shader.id()); }
        }
        unsafe { gl::LinkProgram(id); }

        let mut success : GLint = 1;
        unsafe {
            gl::GetProgramiv(id, gl::LINK_STATUS, &mut success);
        }

        if success == 0 {
            let mut len: gl::types::GLint = 0;
            unsafe {
                gl::GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let error = create_empty_cstring_with_len(len as usize);

            unsafe {
                gl::GetProgramInfoLog(
                    id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar
                );
            }

            return Err(format!("{}",error.to_string_lossy().into_owned()));
        }


        for shader in shaders {
            unsafe { gl::DetachShader(id, shader.id()); }
        }
        
        Ok(Self {
            _id : id,
        })
    
    }

}

impl GlObj for Program {

    fn id(&self) -> GLuint {
        self._id
    }

    fn bind(&self) {
        unsafe {gl::UseProgram(self._id);}
    }
    fn unbind(&self) {
        unsafe {gl::UseProgram(0 as GLuint);}
    }
    
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self._id);
        }
    }
}

// shader struct
pub struct Shader {
    _id : GLuint,
}

impl Shader {

    pub fn from_res(res: &Ressources, name: &str) -> Result<Shader, Error> {
        const POSSIBLE_EXT: [(&str, gl::types::GLenum); 6] = [
            (".vert",gl::VERTEX_SHADER),
            (".tesc",gl::TESS_CONTROL_SHADER),
            (".tese",gl::TESS_EVALUATION_SHADER),
            (".geom",gl::GEOMETRY_SHADER),
            (".frag",gl::FRAGMENT_SHADER),
            (".comp",gl::COMPUTE_SHADER),
        ];

        let stype : GLenum = POSSIBLE_EXT.iter()
        .find(|&&(extension, _)| {
            name.ends_with(extension)
        })
        .map(|&(_, kind)| kind )
        .ok_or(Error::CanNotDetermineShaderTypeForResource{name : name.to_owned()})?;

        let source = res.load_cstring(name)
        .map_err(|e| {Error::ResourceLoad{name : name.to_owned(), inner : e}})?;


        Shader::from_source(source, stype).map_err(|e| {Error::CompileError{name : name.to_owned(), message : e}})
    }

    pub fn from_source(source : CString, shader_type : GLenum) -> Result<Self, String> {
        let id = unsafe {gl::CreateShader(shader_type)};


        unsafe {
            gl::ShaderSource(id, 1, &source[..].as_ptr(), std::ptr::null());
            gl::CompileShader(id);
        }

        // check if the compilation was successfull and print the error message if it isn't.
        let mut success : GLint = 1;
        unsafe {
            gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
        }

        if success == 0 {
            let mut len: gl::types::GLint = 0;
            unsafe {
                gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let error = create_empty_cstring_with_len(len as usize);

            unsafe {
                gl::GetShaderInfoLog(
                    id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar
                );
            }

            return Err( format!("There was an error in the shader compilation: {}",error.to_string_lossy().into_owned()));
        }

        Ok(Self {
            _id : id,
        })
    }


    pub fn id(&self) -> GLuint {
        self._id
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self._id);
        }
    }
}
