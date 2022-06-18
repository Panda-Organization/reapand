use clap::{App, AppSettings, Arg, ColorChoice};
use crate::constants::{CRATE_ABOUT, CRATE_AUTHORS, CRATE_HEADER, CRATE_NAME, CRATE_VERSION};

pub fn base_app() -> App<'static> {
    App::new(CRATE_NAME)
        .version(CRATE_VERSION)
        .author(CRATE_AUTHORS)
        .about(CRATE_ABOUT)
        .before_help(CRATE_HEADER)
        .setting(AppSettings::DeriveDisplayOrder)
        .color(ColorChoice::Always)
        .arg_required_else_help(true)
}