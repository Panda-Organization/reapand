use crate::logger::composite_logger::CompositeLogger;
use crate::logger::console_logger::ConsoleLogger;
use crate::logger::contextual_logger::ContextualLogger;
use crate::logger::file_logger::FileLogger;
use crate::logger::filtered_logger::FilteredLogger;
use crate::logger::logger::Logger;

pub struct LoggerFactory;

//static FILEPATH: Once = Once::new();

/*lazy_static! {
     static ref FILEPATH: Mutex<String> = Mutex::new("files/log.txt".to_string());
}*/

/*pub fn set_filepath(filepath: &mut String) {
    FILEPATH.call_once(|| {

    })
}*/

impl LoggerFactory {
    pub fn get_logger(name: String) -> Box<dyn Logger> {
        Box::new(ContextualLogger {
            delegate_logger: Box::new(CompositeLogger {
                dev_log: Box::new(ConsoleLogger),
                file_log: Box::new(FilteredLogger {
                    delegate_logger: Box::new(FileLogger::new("files/log.txt")),
                    condition: |level| false
                        //matches!(level, Level::ERROR) || matches!(level, Level::WARNING)
                })
            }),
            caller_class: name
        })
    }
}