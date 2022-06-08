mod server;
mod logger;
mod cli;
mod constants;

use std::io::Error;
use colored::control::set_override;
use crate::cli::generate_app;
use crate::constants::APP_MATCHES;
use crate::logger::level::Level;
use crate::logger::logger_factory::LoggerFactory;
use crate::server::handler::handler;

fn main() -> Result<(), Error>{
    set_override(true);
    /*if let Some(e) = matches.values_of(constants::args::encode::NAME) {
        println!("From Main");
        println!("{:?}", e.map(String::from).collect::<Vec<String>>());
    }*/
    let logger = LoggerFactory::get_logger(module_path!().to_string());
    let server = server::new(
        APP_MATCHES.value_of(constants::args::host::NAME).unwrap(),
        APP_MATCHES.value_of(constants::args::port::NAME).unwrap(),
        handler
    );
    logger.log(&format!("Listening on {}", server.server_addr()), &Level::INFO);

    let (handle, sender) = server.stoppable();
    ctrlc::set_handler(move || sender.send(()).unwrap()).expect("Error setting Ctrl-C handler");
    handle.join().unwrap();
    Ok(())
}
