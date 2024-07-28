mod lib;

use crate::{
    lib::{
        help::HELP,
        config::{
            Config,
            ConfigErrors,
            ConfigParsingResult::*,
        }
    }
};

use std::{
    env,
    process,
    error::Error
};

fn main() {

    let args_parsing_result = Config
        ::parse_args(env::args().peekable())
        .unwrap_or_else(|errs| { report_parsing_errors(errs) });

    let config = match args_parsing_result {
        Built(config) => config,
        NeedHelp => { write_help_and_exit() }
    };

    if config.debug_mode {
        dbg!(config);
    }
}

fn report_parsing_errors(errs: ConfigErrors) -> ! {
    for err in errs {
        eprintln!("Error parsing arguments: {err}");
    }
    eprintln!("Use -h to see help.");
    process::exit(1);
}

fn write_help_and_exit() -> ! {
    println!("{HELP}");
    process::exit(0);
}