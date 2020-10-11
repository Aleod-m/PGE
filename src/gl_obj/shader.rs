use gl::types::*;
use std::path::Path;
use std::ffi::{CString};
use std::fs;


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
    
    pub fn new(shaders : &[Shader]) -> Self {
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

            println!("{}",error.to_string_lossy().into_owned());
        }


        for shader in shaders {
            unsafe { gl::DetachShader(id, shader.id()); }
        }
        Self {
            _id : id,
        }
    
    }

    pub fn set_used(&self) {
        unsafe {
            gl::UseProgram(self._id);
        }
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

    pub fn new(path : &Path) -> Self {
        // Find the shader type by checking the file extension
        let stype = match path.extension().and_then(std::ffi::OsStr::to_str) {
            Some("vert") => gl::VERTEX_SHADER,
            Some("tesc") => gl::TESS_CONTROL_SHADER,
            Some("tese") => gl::TESS_EVALUATION_SHADER,
            Some("geom") => gl::GEOMETRY_SHADER,
            Some("frag") => gl::FRAGMENT_SHADER,
            Some("comp") => gl::COMPUTE_SHADER,
            Some(&_) => {
                println!("The extension is not a shader extension!");
                0 as GLenum
            }
            None => {
                println!("The given path is wrong!");
                0 as GLenum
            }
        };
        
        let id = unsafe {gl::CreateShader(stype)};

        let source = Self::parse_shader(path);

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

            println!("{}",error.to_string_lossy().into_owned());
        }

        Self {
            _id : id,
        }
    }

    fn parse_shader(path: &Path) -> CString {
        let source_str = fs::read_to_string(path).unwrap();
        CString::new(source_str).unwrap()
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
