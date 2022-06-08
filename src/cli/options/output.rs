use clap::{App, Arg, ArgGroup, ValueHint};
use crate::constants::{args::*, groups::*};

pub fn add_output(app: App<'static>) -> App<'static> {
    app
        .next_help_heading(output::NAME)
        .arg(
            Arg::new(directory::NAME)
                .value_name(directory::VALUE_NAME)
                .help(directory::HELP)
                .long(directory::LONG)
                .short(directory::SHORT)
                .takes_value(true)
                .default_value(".")
                .value_hint(ValueHint::DirPath)
        )
        .group(
            ArgGroup::new(output::NAME)
                .args(&[
                    directory::NAME
                ])
                .multiple(true)
        )
}
