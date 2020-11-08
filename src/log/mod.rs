mod timer;
pub mod color;
use std::fmt::Debug;
use color::Colors;
use crate::utils::time;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum LogLevel {
    TRACE,
    INFO,
    WARN,
    ERROR,
    FATAL,
}

pub struct Logger {
    name : String,
    colorpicker : Colors,
}


impl Logger{
    pub fn new(name : String) -> Self {
        Self {
            name : name,
            colorpicker : Colors::new(),
        }
    }

    pub fn trace<T : Debug>(&self, content : &T) {
        println!("\x1B[{}m[{}]{}: {:?} \x1B[{}m", self.colorpicker.get_color(0), time::get_time().unwrap(), self.name, content, self.colorpicker.get_color(4));
    }

    pub fn info<T : Debug>(&self, content : &T) {
        println!("\x1B[{}m[{}]{}: {:?} \x1B[{}m", self.colorpicker.get_color(1), time::get_time().unwrap(), self.name, content, self.colorpicker.get_color(4));
    }
    pub fn warn<T : Debug>(&self, content : &T) {
        println!("\x1B[{}m[{}]{}: {:?} \x1B[{}m", self.colorpicker.get_color(2), time::get_time().unwrap(), self.name, content, self.colorpicker.get_color(4));
    }
    pub fn error<T : Debug>(&self, content : &T) {
        println!("\x1B[{}m[{}]{}: {:?} \x1B[{}m", self.colorpicker.get_color(3), time::get_time().unwrap(), self.name, content, self.colorpicker.get_color(4));
    }
    pub fn fatal<T : Debug>(&self, content : &T) -> ! {
        println!("\x1B[{}m[{}]{}: {:?} \x1B[{}m", self.colorpicker.get_color(3), time::get_time().unwrap(), self.name, content, self.colorpicker.get_color(4));
        panic!("Fatal error message!")
    }


}   