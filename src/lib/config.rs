use std::path::Path;

pub enum ArgsParsingResult {
    Built(Config),
    NeedHelp
}

#[derive(Debug)]
pub struct Config {
    search_expr: String,
    sources: Vec<Sources>,
    match_displaying_mode: MatchDisplayingMode
}

#[derive(Debug)]
pub enum Sources {
    PlainText(String),
    FilePath(String),
    DirPath(String),
}

#[derive(Debug)]
pub enum MatchDisplayingMode {
    Up(usize),
    Around(usize),
    Down(usize)
}

impl Config {

    pub fn parse_args<I>(args_iterator: I) -> Result<ArgsParsingResult, &'static str>
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

    const ERR_MESSAGE: &str = "You must provide lines count while using option -l.";

    let str = args_iterator.next().ok_or(ERR_MESSAGE)?;
    let last_char = str.chars().nth_back(0).ok_or(ERR_MESSAGE)?;

    if last_char.is_alphabetic() {
        let parse_chars = &str[..str.len() - 1];
        let lines_count = parse_chars.parse::<usize>().map_err(|_| { ERR_MESSAGE })?;
        match last_char {
            'u' => Ok(Up(lines_count)),
            'd' => Ok(Down(lines_count)),
            _   => Err(ERR_MESSAGE)
        }
    } else {
        Ok(Around(str.parse::<usize>().map_err(|_| { ERR_MESSAGE })?))
    }
}

fn parse_source(arg: String) -> Result<Sources, &'static str> {
    use Sources::*;

    let path = Path::new(&arg);
    let exists = Path::try_exists(path).unwrap_or_else(|_| {
        eprintln!("Cannot stat fs. Path will be threaten as plain text.");
        false
    });

    match exists {
        true => match Path::is_dir(path) {
            true  => Ok(DirPath(arg)),
            false => Ok(FilePath(arg))
        },
        false => Ok(PlainText(arg))
    }
}