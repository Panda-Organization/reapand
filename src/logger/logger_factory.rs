use crate::logger::composite_logger::CompositeLogger;
use crate::logger::console_logger::ConsoleLogger;
use crate::logger::contextual_logger::ContextualLogger;
use crate::logger::file_logger::FileLogger;
use crate::logger::filtered_logger::FilteredLogger;
use crate::logger::level::Level;
use crate::logger::logger::Logger;

pub struct LoggerFactory;

impl LoggerFactory {
    pub fn get_logger(name: String) -> Box<dyn Logger> {
        Box::new(ContextualLogger {
            delegate_logger: Box::new(CompositeLogger {
                dev_log: Box::new(ConsoleLogger),
                file_log: Box::new(FilteredLogger {
                    delegate_logger: Box::new(FileLogger::new("files/log.txt")),
                    condition: |level|
                        matches!(level, Level::ERROR) || matches!(level, Level::WARNING)
                })
            }),
            caller_class: name
        })
    }
}