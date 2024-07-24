mod lib;
use crate::lib::config::Config;
use crate::lib::config::ArgsParsingResult::*;
use crate::lib::help::HELP;

use std::{env, process};
use std::error::Error;

fn main() {

    let args_parsing_result = Config
        ::parse_args(env::args())
        .unwrap_or_else(|err| { report_parsing_error(err) });

    let config = match args_parsing_result {
        Built(config) => config,
        NeedHelp => { write_help_and_exit() }
    };

    dbg!(config);
}

fn report_parsing_error(err: &'static str) -> ! {
    eprintln!("Error parsing arguments: {err}");
    eprintln!("Use -h to see help.");
    process::exit(1);
}

fn write_help_and_exit() -> ! {
    println!("{HELP}");
    process::exit(0);
}