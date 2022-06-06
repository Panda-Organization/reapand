use crate::logger::level::Level;
use crate::logger::logger::Logger;

pub struct CompositeLogger {
    pub(crate) dev_log: Box<dyn Logger>,
    pub(crate) file_log: Box<dyn Logger>
}

impl Logger for CompositeLogger {
    fn log(&self, message: &String, level: &Level) {
        self.dev_log.log(&message, &level);
        self.file_log.log(&message, &level);
    }
}