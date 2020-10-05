use chrono::prelude::*;

pub struct Timer {
    local_date : DateTime<Local>,
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            local_date : Local::now(),
        }
    }

    pub fn get_time(&mut self) -> String {
        self.local_date = Local::now();
        self.local_date.format("%H:%M:%S").to_string()
    }
}