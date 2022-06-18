use clap::{Arg, ArgGroup, Command, ValueHint};
use crate::constants::{args::*, groups::*};

pub fn add_listener(app: Command<'static>) -> Command<'static> {
    app
        .next_help_heading(listener::NAME)
        .arg(
            Arg::new(host::NAME)
                .value_name(host::VALUE_NAME)
                .help(host::HELP)
                .takes_value(true)
                .default_value("0.0.0.0")
                .value_hint(ValueHint::Hostname)
        )
        .arg(
            Arg::new(port::NAME)
                .value_name(port::VALUE_NAME)
                .help(port::HELP)
                .long(port::LONG)
                .default_value("4444")
                .short(port::SHORT)
                .takes_value(true)
        )
        .group(
            ArgGroup::new(listener::NAME)
                .args(&[
                    host::NAME, port::NAME
                ])
                .multiple(true)
        )
}