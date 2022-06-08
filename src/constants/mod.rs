use clap::ArgMatches;
use lazy_static::lazy_static;
use crate::generate_app;

pub mod args;
pub mod groups;

pub const CRATE_NAME: &str = env!("CARGO_PKG_NAME");
pub const CRATE_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const CRATE_AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
pub const CRATE_ABOUT: &str = env!("CARGO_PKG_DESCRIPTION");

lazy_static!{
    pub static ref APP_MATCHES: ArgMatches = generate_app().get_matches();
}

