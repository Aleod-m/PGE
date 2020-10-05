pub extern crate log;
use glfw::Context;

//use gl::{ClearColor};
//use gl::types::*;


pub struct App{
    pub client_logger : log::Logger,
    pub(crate) core_logger : log::Logger,
    window : glfw::Window,
    events : std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>,
    glfw : glfw::Glfw,
}

impl App {

    pub fn new(title : &str, width : u32, height : u32) -> Self {

        //logger initialisation
        let mut client_logger = log::Logger::new(String::from("PGE"));
        client_logger.log(log::LogLevel::INFO, &String::from("Client logger initialized!"));
        let mut core_logger = log::Logger::new(String::from("PGE_Core"));
        core_logger.log(log::LogLevel::INFO, &String::from("Core logger initialized!"));

        //glfw window initialisation
        let glfw_init = glfw::init(glfw::FAIL_ON_ERRORS);
        let glfw : glfw::Glfw;
        match glfw_init {
            Ok(v) =>  glfw = v,
            Err(e) => {core_logger.log(log::LogLevel::FATAL, &e); panic!()},
        };

        let win_innit = glfw.create_window(width, height, title, glfw::WindowMode::Windowed);
        
        let mut window : glfw::Window; let events : std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>;
        match win_innit {
            Some(v) =>  {window = v.0;events = v.1;},
            None => {core_logger.log(log::LogLevel::FATAL, &String::from("Failed to create GLFW window.")); panic!()},
        };

        window.make_current();
        window.set_key_polling(true);
    

        Self {
            client_logger,
            core_logger,
            window,
            events,
            glfw,
        }
    }

    pub fn run(&mut self){
        self.core_logger.log(log::LogLevel::INFO, &String::from("App is Running"));
        // Loop until the user closes the window
        while !self.window.should_close() {
            // Swap front and back buffers
            self.window.swap_buffers();

            // Poll for and process events
            self.glfw.poll_events();
            for (_, event) in glfw::flush_messages(&self.events) {
                self.core_logger.log(log::LogLevel::INFO, &event);
                match event {
                    glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) => {
                        self.window.set_should_close(true)
                    },
                    _ => {},
                }
            }
        }
    }
}
