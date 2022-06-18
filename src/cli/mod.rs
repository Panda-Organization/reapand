use clap::Command;
use base_app::base_app;
use crate::cli::options::encoding::add_encoding;
use crate::cli::options::footer::add_footer;
use crate::cli::options::listener::add_listener;
use crate::cli::options::output::add_output;

mod base_app;
mod options;

pub fn generate_app() -> Command<'static> {
    add_footer(
        add_output(
            add_listener(
                add_encoding(
                    base_app()
                )
            )
        )
    )
}