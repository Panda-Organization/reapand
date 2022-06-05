use crate::logger::level::Level;

pub trait Logger {
    fn log(message: String, level: Level);
}