extern crate core;

mod server;
mod logger;

use std::io::Error;

fn main() -> Result<(), Error>{
    let s = server::new("0.0.0.0", "20000");
    Ok(())
}
