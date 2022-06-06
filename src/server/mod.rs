pub mod handler;
pub mod status;

use rouille::{Request, Response, Server};
use crate::{Level, LoggerFactory};


pub fn new(ip: &str, port: &str, handler: fn(&Request) -> Response) -> Server<fn(&Request) -> Response> {
    let logger = LoggerFactory::get_logger(
        [module_path!(), stringify!(main)].join("::")
    );

    match Server::new([ip, port].join(":"), handler) {
        Ok(server) => {
            server
        }
        Err(err) => {
            logger.log(&err.to_string(), &Level::ERROR);
            panic!()
        }
    }
}