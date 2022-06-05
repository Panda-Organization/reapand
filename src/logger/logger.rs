use crate::logger::level::Level;

pub trait Logger {
    fn log(&self, message: &String, level: &Level);
}