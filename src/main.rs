mod help;
mod config;

use std::{env, process};
use std::error::Error;

use crate::config::Config;
use crate::config::ArgsParsingResult::*;
use crate::help::HELP;

fn main() {

    let args_parsing_result = Config::parse_args(env::args())
        .unwrap_or_else(|err| { report_parsing_error(err) });

    let config = match args_parsing_result {
        Built(config) => config,
        NeedHelp => { write_help_and_exit() }
    };

    dbg!(config);
}

fn report_parsing_error(err: Box<dyn Error>) -> ! {
    eprintln!("Error parsing arguments: {err}");
    process::exit(1);
}

fn write_help_and_exit() -> ! {
    println!("{HELP}");
    process::exit(0);
}