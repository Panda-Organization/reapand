use tiny_http::Server;
use crate::{Level, LoggerFactory};


pub fn new(ip: &str, port: &str) -> Server {
    let logger = LoggerFactory::get_logger(
        [module_path!(), stringify!(main)].join("::")
    );

    match Server::http([ip, port].join(":")) {
        Ok(server) => {
            server
        }
        Err(err) => {
            logger.log(&err.to_string(), &Level::ERROR);
            panic!()
        }
    }
}