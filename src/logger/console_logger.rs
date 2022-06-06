use colored::Colorize;
use crate::logger::level::Level;
use crate::logger::logger::Logger;

pub struct ConsoleLogger;
impl Logger for ConsoleLogger {
    fn log(&self, message: &String, level: &Level) {
        match level {
            Level::ERROR => {
                eprintln!(
                    "[{}] {}",
                    (&*(level.to_string())).on_red(),
                    message
                )
            }
            Level::WARNING => {
                println!(
                    "[{}] {}",
                    (&*(level.to_string())).bright_yellow(),
                    message
                )
            }
            Level::INFO => {
                println!(
                    "[{}] {}",
                    (&*(level.to_string())).blue(),
                    message
                )
            }
            Level::CRITICAL => {}
            Level::EXCEPTION => {}
            Level::FAILURE => {}
            Level::HEXDUMP => {}
            Level::INDENTED => {}
            Level::PROGRESS => {}
            Level::SUCCESS => {}
            Level::WAITFOR => {}
        }
    }
}