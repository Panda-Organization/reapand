use crate::logger::level::Level;
use crate::logger::logger::Logger;

pub struct FilteredLogger<'a> {
    pub(crate) delegate_logger: &'a dyn Logger,
    pub(crate) condition: fn(level: &Level) -> bool
}

impl Logger for FilteredLogger<'_> {
    fn log(&self, message: &String, level: &Level) {
        if (self.condition)(level) {
            &self.delegate_logger.log(message, level);
        }
    }
}