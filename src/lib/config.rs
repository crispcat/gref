use std::{
    sync::Arc,
    path::Path,
    str::FromStr,
    iter::Peekable,
    num::NonZeroUsize,
    thread::available_parallelism,
};

pub type PathString = String;
pub type ConfigError = String;
pub type ConfigErrors = Vec<ConfigError>;


#[derive(Debug, Default)]
pub struct Config {
    pub search_expr:                      String,
    pub sources:                          Vec<Arc<TextSource>>,
    pub groups_to_extract:                Vec<String>,          // -e
    pub output_format:                    String,               // -f
    pub threads:                          usize,                // -t
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
    FilePath(PathString),
    DirPath(PathString),
    Stdin
}

pub enum ConfigParsingResult {
    Built(Config),
    NeedHelp
}

impl Config {

    pub fn parse_args<I>(args_iterator: Peekable<I>) -> Result<ConfigParsingResult, ConfigErrors>
    where
        I: Iterator<Item=String>,
    {
        use ConfigParsingResult::*;
        use TextSource::*;

        let mut errors = vec![];

        let mut config = Config::default();
        let mut search_expr: Option<String> = None;
        let mut output_format: Option<String> = None;
        let mut threads_count: Option<NonZeroUsize> = None;
        let mut args_iterator = args_iterator.skip(1).peekable();

        while let Some(arg) = args_iterator.peek() {
            match arg.as_str() {
                "-h" => {
                    return Ok(NeedHelp)
                },
                "-e" => {
                    match parse_param_value(&mut args_iterator) {
                        Ok(val) => config.groups_to_extract.push(val),
                        Err(err) => errors.push(err)
                    }
                },
                "-f" => {
                    match parse_param_value(&mut args_iterator) {
                        Ok(val) => output_format = Some(val),
                        Err(err) => errors.push(err)
                    }
                },
                "-p" => {
                    match parse_param_value(&mut args_iterator) {
                        Ok(val) => config.sources.push(Arc::new(PlainText(val))),
                        Err(err) => errors.push(err)
                    }
                },
                "-t" => {
                    match parse_param_value(&mut args_iterator) {
                        Ok(val) => threads_count = Some(val),
                        Err(err) => errors.push(err)
                    }
                },
                "-v" => {
                    config.verbose = true;
                    args_iterator.next();
                },
                "-i" => {
                    config.case_insensitive = true;
                    args_iterator.next();
                },
                "-m" => {
                    config.multiline = true;
                    args_iterator.next();
                },
                "-s" => {
                    config.dot_matches_new_line = true;
                    args_iterator.next();
                },
                "-U" => {
                    config.swap_greed = true;
                    args_iterator.next();
                },
                "-x" => {
                    config.ignore_whitespace_and_comments = true;
                    args_iterator.next();
                },
                "-u" => {
                    config.unicode = true;
                    args_iterator.next();
                },
                "-d" => {
                    config.debug_mode = true;
                    args_iterator.next();
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
                        None => search_expr = Some(arg),
                        Some(_) => match stat_fs_source(arg) {
                            Ok(source) => config.sources.push(Arc::new(source)),
                            Err(err) => errors.push(err)
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

        // TODO: come with more stable and perfomant thread spawning strategy
        config.threads = threads_count
            .unwrap_or(available_parallelism()
                .unwrap_or(NonZeroUsize::new(1).unwrap()))
            .get();

        config.output_format = output_format.unwrap_or(String::from("{0}"));
        config.sources.push(Arc::new(Stdin));

        if errors.len() == 0 {
            Ok(Built(config))
        } else {
            Err(errors)
        }
    }
}

fn parse_param_value<I, T>(args_iterator: &mut Peekable<I>) -> Result<T, ConfigError>
where
    I: Iterator<Item=String>,
    T: FromStr
{
    let key = args_iterator.next().unwrap();
    let error_message = format!("You must provide a value when using option {key}");

    let arg = args_iterator.peek().ok_or(&error_message)?;
    if arg.starts_with('-') {
        return Err(error_message);
    }

    let val = args_iterator.next().unwrap();
    val.parse::<T>().map_err(|_| format!("Cannot parse value {val} of argument {key}"))
}

fn stat_fs_source(arg: String) -> Result<TextSource, ConfigError> {
    use TextSource::*;
    let path = Path::new(&arg);
    let exists = Path::try_exists(path).map_err(|e| { format!("Cannot stat fs.\n{e}") })?;
    match exists {
        true => match Path::is_dir(path) {
            true => Ok(DirPath(arg)),
            false => Ok(FilePath(arg))
        }
        false => Err(format!("File or directory {arg} is not exist!"))
    }
}