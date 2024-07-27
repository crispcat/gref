use std::{
    error::Error,
    io::{
        BufReader,
        Read
    }
};

use crate::lib::config::{
    Config,
    TextSource
};

use regex::bytes::RegexBuilder;

struct SearchResult {
    line: usize,
    position: usize,
    needle: String,
    haystack: TextSource
}

pub fn run(config: &Config) -> Result<Vec<SearchResult>, Box<dyn Error>> {

    let search_results = Vec::<SearchResult>::new();
    let regex_builder = RegexBuilder
        ::new(config.search_expr.as_str())
        .case_insensitive(config.case_insensitive)
        .multi_line(config.multiline)
        .dot_matches_new_line(config.dot_matches_new_line)
        .swap_greed(config.swap_greed)
        .ignore_whitespace(config.ignore_whitespace_and_comments)
        .unicode(config.unicode)
        .build()?;

    for source in config.sources.iter() {

    }

    Ok(search_results)
}