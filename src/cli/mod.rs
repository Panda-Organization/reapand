use clap::App;
use base_app::base_app;
use crate::cli::options::encoding::add_encoding;

mod base_app;
mod options;

pub fn generate_app() -> App<'static> {
    add_encoding(
        base_app()
    )
}