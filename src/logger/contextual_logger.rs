use chrono::Utc;
use colored::Colorize;
use crate::logger::level::Level;
use crate::logger::logger::Logger;

const DATE_FORMAT: &str = "%Y-%m-%d %H:%M:%S%.3f";

pub struct ContextualLogger {
    pub(crate) caller_class: String,
    pub(crate) delegate_logger: Box<dyn Logger>
}

impl Logger for ContextualLogger {
    fn log(&self, message: &String, level: &Level) {
        self.delegate_logger.log(&format!(
            "{} {}: {}",
            Utc::now().format(DATE_FORMAT).to_string().as_str().green(),
            self.caller_class.as_str().yellow(),
            message
        ), level)
    }
}