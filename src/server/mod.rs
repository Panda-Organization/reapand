use std::process::exit;
use tiny_http::Server;

pub fn new(ip: &str, port: &str) -> Server {
    match Server::http([ip, port].join(":")) {
        Ok(server) => {
            server
        }
        Err(err) => {
            panic!()
        }
    }
}