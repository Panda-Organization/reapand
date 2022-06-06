mod server;
mod logger;

use std::io::Error;
use colored::control::set_override;
use crate::logger::level::Level;
use crate::logger::logger_factory::LoggerFactory;

fn main() -> Result<(), Error>{
    set_override(true);
    let logger = LoggerFactory::get_logger(
        [module_path!(), stringify!(main)].join("::")
    );
    let s = server::new("0.0.0.0", "20000");
    logger.log(&format!("Listening on {}", s.server_addr()), &Level::INFO);
    logger.log(&"Coucou".to_string(), &Level::WARNING);
    logger.log(&"Alo".to_string(), &Level::ERROR);
    Ok(())
}
