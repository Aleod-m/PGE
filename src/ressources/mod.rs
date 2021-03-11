// Extern imports
use std::ffi;
use std::fs;
use std::io::{self, Read};
use std::{
    path::{Path, PathBuf},
    rc::Rc,
};

// Module imports

#[derive(Debug)]
pub enum Error {
    FailedToGetPath,
    FileContainsNullByte,
    IO(io::Error),
}

#[derive(Clone)]
pub struct RessourceLoader {
    root_dir: Rc<PathBuf>,
}

impl RessourceLoader {
    /// Create the ressource loader struct capable of loding files from a relative
    /// path to the executable
    pub fn init(rel_path: &Path) -> Result<RessourceLoader, Error> {
        let exe_name = std::env::current_exe().map_err(|_| Error::FailedToGetPath)?;
        let exe_path = exe_name.parent().ok_or(Error::FailedToGetPath)?;
        Ok(Self {
            root_dir: Rc::new(exe_path.join(rel_path)),
        })
    }

    /// Load the rssource as a Cstring.
    pub fn load_cstring(&self, name: &str, exts : &[&str]) -> Result<ffi::CString, Error> {
        let mut file = fs::File::open(self.root_dir.join(name))?;

        let mut content: Vec<u8> = Vec::with_capacity(file.metadata()?.len() as usize + 1);
        file.read_to_end(&mut content)?;

        if content.iter().find(|i| **i == 0).is_some() {
            return Err(Error::FileContainsNullByte);
        }

        Ok(unsafe { ffi::CString::from_vec_unchecked(content) })
    }

<<<<<<< HEAD
    pub fn load_bytes(&self, path : &str) -> Result<Vec<u8>, Error> {
        let mut file = fs::File::open(self.path.join(path))?;
=======
    /// Load the rssource as a vector of bytes.
    pub fn load_bytes(&self, name: &str, exts : &[&str]) -> Result<Vec<u8>, Error> {
        let mut file = fs::File::open(self.root_dir.join(name))?;
>>>>>>> ce189835fa89a1806beb5432e49e7cf048a91ada

        let mut bytes: Vec<u8> = Vec::with_capacity(file.metadata()?.len() as usize + 1);
        file.read_to_end(&mut bytes)?;
        Ok(bytes)
    }

    pub fn load_as_reader(&self, name: &str, exts : &[&str]) -> Result<impl Read, Error> {
        fs::File::open(self.root_dir.join(name)).map_err(|e| Error::IO(e))
    }

    pub fn name_to_path(&self, name : &str) -> PathBuf {
        let path = self.root_dir.clone();
        path.join(name)
    }
}

impl From<io::Error> for Error {
    fn from(other: io::Error) -> Self {
        Error::IO(other)
    }
}