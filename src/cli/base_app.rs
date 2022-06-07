use clap::{App, AppSettings, Arg};
use crate::constants::{CRATE_ABOUT, CRATE_AUTHORS, CRATE_NAME, CRATE_VERSION};

pub fn base_app() -> App<'static> {
    App::new(CRATE_NAME)
        .version(CRATE_VERSION)
        .author(CRATE_AUTHORS)
        .about(CRATE_ABOUT)
        .setting(AppSettings::DeriveDisplayOrder)
        .arg_required_else_help(true)
}