mod server;
mod logger;

use std::io::Error;
use colored::control::set_override;
use crate::logger::level::Level;
use crate::logger::logger_factory::LoggerFactory;
use crate::server::handler::handler;

fn main() -> Result<(), Error>{
    set_override(true);
    let logger = LoggerFactory::get_logger(module_path!().to_string());
    let server = server::new("0.0.0.0", "20000", handler);
    logger.log(&format!("Listening on {}", server.server_addr()), &Level::INFO);

    let (handle, sender) = server.stoppable();
    ctrlc::set_handler(move || sender.send(()).unwrap()).expect("Error setting Ctrl-C handler");
    //logger.log(&"Coucou".to_string(), &Level::WARNING);
    //logger.log(&"Alo".to_string(), &Level::ERROR);
    handle.join().unwrap();
    Ok(())
}
