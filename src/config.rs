use std::error::Error;
use std::path::Path;

use const_format::formatcp;
use crate::help::OPTION_HELP_L;

pub (crate) enum ArgsParsingResult {
    Built(Config),
    NeedHelp
}

#[derive(Debug)]
pub(crate) struct Config {
    search_expr: String,
    sources: Vec<Sources>,
    match_displaying_mode: MatchDisplayingMode
}

#[derive(Debug)]
pub(crate) enum Sources {
    PlainText(String),
    FilePath(String),
    DirPath(String),
}

#[derive(Debug)]
pub(crate) enum MatchDisplayingMode {
    Up(usize),
    Around(usize),
    Down(usize)
}

impl Config {

    pub(crate)
    fn parse_args<I>(args_iterator: I) -> Result<ArgsParsingResult, Box<dyn Error>>
    where
        I: Iterator<Item=String>, {
        use ArgsParsingResult::*;

        let mut search_expr: Option<String> = None;
        let mut sources = Vec::<Sources>::new();
        let mut match_displaying_mode: Option<MatchDisplayingMode> = None;

        let mut args_iterator = args_iterator.skip(1);

        while let Some(arg) = args_iterator.next() {
            match arg.as_str() {
                "-h" => {
                    return Ok(NeedHelp)
                },
                "-l" => {
                    match_displaying_mode = Some(parse_match_displaying_mode(&mut args_iterator)?)
                },
                _ => {
                    match search_expr {
                        None    => { search_expr = Some(arg) }
                        Some(_) => { sources.push(parse_source(arg)?) }
                    }
                }
            }
        }

        let search_expr = search_expr
            .ok_or("Insufficient arguments. You must provide at least a search expression.")?;

        let match_displaying_mode = match_displaying_mode
            .unwrap_or(MatchDisplayingMode::Around(4));

        Ok(Built(Config {
            search_expr,
            sources,
            match_displaying_mode,
        }))
    }
}

fn parse_match_displaying_mode<I>(args_iterator: &mut I) -> Result<MatchDisplayingMode, &'static str>
where
    I: Iterator<Item=String>, {
    use MatchDisplayingMode::*;

    const ERR_MESSAGE: &str = formatcp!("You must provide lines count while using option -l.\n{OPTION_HELP_L}");

    let str = args_iterator.next().ok_or(ERR_MESSAGE)?;
    let last_char = str.chars().nth_back(0).ok_or(ERR_MESSAGE)?;

    if last_char.is_alphabetic() {
        let parse_chars = &str[..str.len() - 1];
        let count = parse_chars.parse::<usize>().map_err(|_| { ERR_MESSAGE })?;
        match last_char {
            'u' => Ok(Up(count)),
            'd' => Ok(Down(count)),
            _   => Err(ERR_MESSAGE)
        }
    } else {
        Ok(Around(str.parse::<usize>().map_err(|_| { ERR_MESSAGE })?))
    }
}

fn parse_source(arg: String) -> Result<Sources, Box<dyn Error>> {
    use Sources::*;
    let path = Path::new(&arg);
    if Path::try_exists(path)? {
        if Path::is_dir(path) {
            Ok(DirPath(arg))
        } else {
            Ok(FilePath(arg))
        }
    } else {
        Ok(PlainText(arg))
    }
}