use clap::{Arg, ArgGroup, Command};
use crate::constants::{args::*, groups::*};

pub fn add_encoding(app: Command<'static>) -> Command<'static> {
    app
        .next_help_heading(encoding::NAME)
        .arg(
            Arg::new(encode::NAME)
                .long(encode::LONG)
                .value_name(encode::VALUE_NAME)
                .help(encode::HELP)
                .takes_value(true)
                .possible_values(encode::POSSIBLE_VALUES)
                .multiple_values(true)
                .use_value_delimiter(true)
                .short(encode::SHORT)
                .required(true)
        )
        .group(
            ArgGroup::new(encoding::NAME)
                .args(&[
                    encode::NAME
                ])
                .multiple(true)
        )
}