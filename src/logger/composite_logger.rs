use crate::logger::level::Level;
use crate::logger::logger::Logger;

pub struct CompositeLogger<'a> {
    pub(crate) dev_log: &'a dyn Logger,
    pub(crate) file_log: &'a dyn Logger
}

impl Logger for CompositeLogger<'_> {
    fn log(&self, message: &String, level: &Level) {
        self.dev_log.log(&message, &level);
        self.file_log.log(&message, &level);
    }
}