use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use crate::logger::level::Level;
use crate::logger::logger::Logger;

pub struct FileLogger {
    path: String
}

impl FileLogger {
    pub fn new(path_as_string: &str) -> Self {
        if let Ok(absolute_path) = fs::canonicalize(path_as_string) {
            return Self {
                path: absolute_path.into_os_string().into_string().unwrap()
            }
        }
        panic!("Error: The path does not exist")
    }
}

impl Logger for FileLogger {
    fn log(&self, message: &String, level: &Level) {
        let mut f = OpenOptions::new()
            .write(true)
            .append(true)
            .open(&self.path)
            .expect(&*format!("Unable to open file [{}]", &self.path));
        f.write_all((level.to_string() + ": " + message + "\n").as_bytes())
            .expect(&*format!("Cannot write log message to file [{}]", &self.path))
    }
}