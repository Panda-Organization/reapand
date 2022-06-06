use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use path_absolutize::Absolutize;
use crate::logger::level::Level;
use crate::logger::logger::Logger;

pub struct FileLogger {
    path: String
}

impl FileLogger {
    pub fn new(path_as_string: &str) -> Self {
        return Self {
            path: Path::new(path_as_string).absolutize().unwrap().to_str().unwrap().to_string()
        }
    }
}

impl Logger for FileLogger {
    fn log(&self, message: &String, level: &Level) {
        let mut f = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)
            .expect(&*format!("Unable to open file [{}]", &self.path));
        f.write_all((level.to_string() + ": " + message + "\n").as_bytes())
            .expect(&*format!("Cannot write log message to file [{}]", &self.path))
    }
}