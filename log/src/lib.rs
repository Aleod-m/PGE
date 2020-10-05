mod timer;
pub mod color;
use std::fmt::Debug;
use color::Colors;
use timer::Timer;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum LogLevel {
    TRACE,
    INFO,
    WARN,
    ERROR,
    FATAL
}

pub struct Logger {
    name : String,
    colorpicker : Colors,
    time : Timer,
}

impl Logger{
    pub fn new(name : String) -> Self {
        Self {
            name : name,
            colorpicker : Colors::new(),
            time : Timer::new(),
        }
    }

    pub fn log<T : Debug>(&mut self, log_level: LogLevel, content : &T) {
        let log_color = match log_level {
            LogLevel::TRACE => self.colorpicker.get_color(0),
            LogLevel::INFO  => self.colorpicker.get_color(1),
            LogLevel::WARN  => self.colorpicker.get_color(2),
            LogLevel::ERROR => self.colorpicker.get_color(3),
            LogLevel::FATAL => self.colorpicker.get_color(3)
        };
        println!("\x1B[{}m[{}]{}: {:?} \x1B[{}m", log_color, self.time.get_time(), self.name, content, self.colorpicker.get_color(4));
        if log_level == LogLevel::FATAL {
            panic!("Fatal error message!");
        }
    }
}   