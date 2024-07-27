use std::{
    iter::Peekable,
    path::Path
};

#[derive(Debug, Default)]
pub struct Config {
    pub search_expr:                      String,
    pub sources:                          Vec<TextSource>,
    pub groups_to_extract:                Vec<String>,          // -e
    pub output_format:                    String,               // -f
    pub verbose:                          bool,                 // -v
    pub case_insensitive:                 bool,                 // -i
    pub multiline:                        bool,                 // -m
    pub dot_matches_new_line:             bool,                 // -s
    pub swap_greed:                       bool,                 // -U
    pub ignore_whitespace_and_comments:   bool,                 // -x
    pub unicode:                          bool,                 // -u
    pub debug_mode:                       bool                  // -d
}

#[derive(Debug)]
pub enum TextSource {
    PlainText(String),
    FilePath(String),
    DirPath(String),
    Stdin
}

pub enum ArgsParsingResult {
    Built(Config),
    NeedHelp
}

impl Config {

    pub fn parse_args<I>(args_iterator: Peekable<I>) -> Result<ArgsParsingResult, Vec<String>>
    where
        I: Iterator<Item=String>,
    {
        use ArgsParsingResult::*;
        use TextSource::*;

        let mut errors = vec![];

        let mut config = Config::default();
        let mut search_expr: Option<String> = None;
        let mut output_format: Option<String> = None;
        let mut args_iterator = args_iterator.skip(1).peekable();

        while let Some(arg) = args_iterator.peek() {
            match arg.as_str() {
                "-h" => {
                    return Ok(NeedHelp)
                },
                "-e" => {
                    match parse_param_value(&mut args_iterator) {
                        Ok(val) => config.groups_to_extract.push(val),
                        Err(er) => errors.push(er)
                    }
                },
                "-f" => {
                    match parse_param_value(&mut args_iterator) {
                        Ok(val) => output_format = Some(val),
                        Err(er) => errors.push(er)
                    }
                },
                "-p" => {
                    if let Ok(val) = parse_param_value(&mut args_iterator)
                        .map_err(|e| errors.push(e)) {
                        config.sources.push(PlainText(val))
                    }
                },
                "-v" => {
                    config.verbose = args_iterator.next().unwrap().map(|| true);
                },
                "-i" => {
                    config.case_insensitive = args_iterator.next().unwrap().map(|| true)
                },
                "-m" => {
                    config.multiline = args_iterator.next().unwrap().map(|| true)
                },
                "-s" => {
                    config.dot_matches_new_line = args_iterator.next().unwrap().map(|| true)
                },
                "-U" => {
                    config.swap_greed = args_iterator.next().unwrap().map(|| true)
                },
                "-x" => {
                    config.ignore_whitespace_and_comments = args_iterator.next().unwrap().map(|| true)
                },
                "-u" => {
                    config.unicode = args_iterator.next().unwrap().map(|| true)
                },
                "-d" => {
                    config.debug_mode = args_iterator.next().unwrap().map(|| true)
                },
                _ => {
                    let arg = args_iterator.next().unwrap();
                    if arg.starts_with('-') {
                        errors.push(format!("Incorrect option {arg}. \
                        If you want to use {arg} as search expression \
                        you can force it to be treated as regex using \"({arg})\"."));
                        continue;
                    }
                    match search_expr {
                        None    => search_expr = Some(arg),
                        Some(_) => match stat_fs_source(arg) {
                            Ok(source) => config.sources.push(source),
                            Err(error) => errors.push(error)
                        }
                    }
                }
            }
        }

        match search_expr {
            Some(expr) => config.search_expr = expr,
            None => errors.push(String::from("Insufficient arguments. \
                You must provide at least a search expression."))
        }

        config.output_format = output_format.unwrap_or(String::from("{0}"));
        config.sources.push(Stdin);

        if errors.len() == 0 {
            Ok(Built(config))
        } else {
            Err(errors)
        }
    }
}

fn parse_param_value<I>(args_iterator: &mut Peekable<I>) -> Result<String, String>
where
    I: Iterator<Item=String>,
{
    let key = args_iterator.next();
    let error_message = format!("You must provide a value when using option {key}");

    let arg = args_iterator.peek().ok_or(&error_message)?;
    if arg.starts_with('-') {
        return Err(error_message);
    }

    Ok(args_iterator.next().unwrap())
}

fn stat_fs_source(arg: String) -> Result<TextSource, String> {
    use TextSource::*;
    let path = Path::new(&arg);
    let exists = Path::try_exists(path).map_err(|e| { format!("Cannot stat fs.\n{e}") })?;
    match exists {
        true => match Path::is_dir(path) {
            true  => Ok(DirPath(arg)),
            false => Ok(FilePath(arg))
        }
        false => Err(format!("File or directory {arg} is not exist!"))
    }
}