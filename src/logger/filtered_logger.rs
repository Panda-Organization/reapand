use crate::logger::level::Level;
use crate::logger::logger::Logger;

pub struct FilteredLogger {
    pub(crate) condition: fn(level: &Level) -> bool,
    pub(crate) delegate_logger: Box<dyn Logger>
}

impl Logger for FilteredLogger {
    fn log(&self, message: &String, level: &Level) {
        if (self.condition)(level) {
            self.delegate_logger.log(message, level);
        }
    }
}