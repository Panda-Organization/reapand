use clap::Command;
use crate::constants::CRATE_AFTER_HELP;

pub fn add_footer(app: Command<'static>) -> Command<'static> {
    app
        .after_help(CRATE_AFTER_HELP)
}