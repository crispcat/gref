use std::io::Read;

use anyhow::{
    bail,
    anyhow,
};

use regex::bytes::{
    Regex,
    RegexBuilder
};

use crate::lib::{
    config::{
        Config,
        TextSource,
    },
    jobs::JobsChan,
};
use crate::lib::reader::text_reader_wrap;

pub struct SearchRequest<'a> {
    line: usize,
    haystack: String,
    source: &'a TextSource
}

pub struct SearchResult<'a> {
    line: usize,
    source: &'a TextSource,
    position: usize,
    rmatch: String,
}

enum RuntimeJob<'a> {
    ConsumeText(&'a TextSource),
    Search(SearchRequest<'a>),
    Format(SearchResult<'a>)
}

struct Runtime<'a> {
    regex: Regex,
    config: Config,
    jobs_chan: JobsChan<RuntimeJob<'a>>,
}

const DEFAULT_CHAN_CAPACITY: usize = 4096;

pub fn run(config: Config) -> Result<(), anyhow::Error> {
    use RuntimeJob::*;

    let regex_build_result = RegexBuilder
    ::new(config.search_expr.as_str())
        .case_insensitive(config.case_insensitive)
        .multi_line(config.multiline)
        .dot_matches_new_line(config.dot_matches_new_line)
        .swap_greed(config.swap_greed)
        .ignore_whitespace(config.ignore_whitespace_and_comments)
        .unicode(config.unicode)
        .build();

    let regex = regex_build_result.map_err(|e| anyhow!("Regex parsing error: {e})"))?;
    let jobs_chan = JobsChan::<RuntimeJob>::with_capacity(DEFAULT_CHAN_CAPACITY);
    let runtime = Runtime { regex, config, jobs_chan };

    for source in &runtime.config.sources {
        runtime.jobs_chan.announce_one(ConsumeText(source));
    }

    std::thread::scope(|s| {
        for _ in 0..runtime.config.threads {
            s.spawn(|| worker_go(&runtime));
        }
        runtime.jobs_chan.wait_for_all_done();
    });

    Ok(())
}

fn worker_go<'a>(runtime: &'a Runtime<'a>) {
    use RuntimeJob::*;
    while let Some(job_handler) = runtime.jobs_chan.wait_for_one() {
        let job_result = match job_handler.job() {
            ConsumeText(source) => job_consume_text_source(source, runtime),
            Search(request) => job_search_in_string(request, runtime),
            Format(result) => job_format_result(result, runtime)
        };
        job_result.unwrap_or_else(runtime_error)
    }
}

fn job_consume_text_source<'a>(source: &'a TextSource, runtime: &'a Runtime<'a>) -> Result<(), anyhow::Error> {
    use RuntimeJob::*;
    match source {
        TextSource::DirPath(path) => {

        },
        source => {
            let mut line = 0usize;
            let mut text_reader = text_reader_wrap(source, runtime.config.multiline)
                .map_err(|e| anyhow!("Failed to create reader for source \"{source:?}\":{e}"))?;
            loop {
                let mut haystack = String::new();
                match text_reader.read_string_to_buff(&mut haystack) {
                    Ok(0) => break,
                    Ok(_bytes_read) => {
                        let job = Search(SearchRequest { haystack, source, line } );
                        runtime.jobs_chan.announce_one(job);
                    }
                    Err(e) => bail!("Failed to read string from source \"{source:?}\": {e}")
                }
                line += 1;
            }
        }
    }
    Ok(())
}

fn job_search_in_string(req: &SearchRequest, runtime: &Runtime) -> Result<(), anyhow::Error> {
    Ok(())
}

fn job_format_result(res: &SearchResult, runtime: &Runtime) -> Result<(), anyhow::Error> {
    Ok(())
}

fn runtime_error(err: anyhow::Error) {
    eprintln!("Runtime error in worker thread: {}", err);
}