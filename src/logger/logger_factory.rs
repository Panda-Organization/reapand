use crate::logger::composite_logger::CompositeLogger;
use crate::logger::console_logger::ConsoleLogger;
use crate::logger::contextual_logger::ContextualLogger;
use crate::logger::file_logger::FileLogger;
use crate::logger::filtered_logger::FilteredLogger;
use crate::logger::level::Level;
use crate::logger::logger::Logger;

pub struct LoggerFactory;

impl LoggerFactory {
    // FIX HERE
    fn get_logger<'a>(name: String) -> ContextualLogger<'a> {
        ContextualLogger {
            delete_logger: &CompositeLogger {
                dev_log: &ConsoleLogger,
                file_log: &FilteredLogger {
                    delegate_logger: &FileLogger::new("files/log.txt"),
                    condition: |level|
                        matches!(level, Level::ERROR) || matches!(level, Level::WARNING)
                }
            },
            caller_class: name
        }
    }
}