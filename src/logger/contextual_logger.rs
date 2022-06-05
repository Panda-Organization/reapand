use chrono::Utc;
use colored::Colorize;
use crate::logger::level::Level;
use crate::logger::logger::Logger;

const DATE_FORMAT: &str = "YYYY-MM-dd HH:mm:ss.SSS";

pub struct ContextualLogger<'a> {
    pub(crate) delete_logger: &'a dyn Logger,
    pub(crate) caller_class: String
}

impl Logger for ContextualLogger<'_> {
    fn log(&self, message: &String, level: &Level) {
        self.delete_logger.log(&format!(
            "{} {}: {}",
            Utc::now().format(DATE_FORMAT).to_string().as_str().green(),
            self.caller_class.as_str().yellow(),
            message
        ), level)
    }
}