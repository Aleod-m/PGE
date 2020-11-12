// Extern imports
use std::path::{Path, PathBuf};
use std::fs;
use std::io::{self, Read};
use std::ffi;
// Module imports
pub mod image;






#[derive(Debug)]
pub enum Error {
    FailedToGetPath,
    FileContainsNullByte,
    IO(io::Error)
}

pub struct Ressources {
    path : PathBuf,
}

impl Ressources {

    pub fn from_rel_path(rel_path : &Path) -> Result<Ressources, Error> {
        let exe_name = std::env::current_exe().map_err(|_| Error::FailedToGetPath)?;
        let exe_path = exe_name.parent().ok_or(Error::FailedToGetPath)?;
        Ok(Self {
            path : exe_path.join(rel_path),
        })
    }

    pub fn name_to_path(root_dir : &Path, location : &str) -> PathBuf {
        let mut path: PathBuf = root_dir.into();

        for part in location.split("/") {
            path = path.join(part);
        }
    
        path
    }

    pub fn load_cstring(&self, name : &str) -> Result<ffi::CString, Error> {
        let mut file = fs::File::open(self.path.join(name))?;

        let mut content: Vec<u8> = Vec::with_capacity(
            file.metadata()?.len() as usize + 1
        );
        file.read_to_end(&mut content)?;

        if content.iter().find(|i| **i == 0).is_some() {
            return Err(Error::FileContainsNullByte);
        }

        Ok(
            unsafe {
                ffi::CString::from_vec_unchecked(content)
            }
        )
    }

    pub fn load_bytes(&self, name : &str) -> Result<Vec<u8>, Error>{
        let mut file = fs::File::open(self.path.join(name))?;

        let mut bytes: Vec<u8> = Vec::with_capacity(
            file.metadata()?.len() as usize + 1
        );
        file.read_to_end(&mut bytes)?;
        Ok(bytes)
    }
}


impl From<io::Error> for Error {
    fn from(other: io::Error) -> Self {
        Error::IO(other)
    }
}
