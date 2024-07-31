use std::{
    sync::{
        Arc,
        Mutex
    },
};

use regex::bytes::{
    Regex,
    RegexBuilder
};

use crate::{
    lib::{
        jobs::JobsChan,
        config::{
            Config,
            TextSource,
        }
    }
};

pub struct SearchResult {
    line: usize,
    position: usize,
    needle: String,
    haystack: Arc<TextSource>
}

enum RuntimeJob {
    Iterate(Arc<TextSource>),
    SearchInString(String),
    FormatResult(String)
}

struct Runtime {
    regex: Regex,
    config: Config,
    jobs: JobsChan<RuntimeJob>,
    results: Mutex<Vec<SearchResult>>,
}

type RuntimeError = String;
type RunResult = Result<(), Vec<RuntimeError>>;

const DEFAULT_CHAN_CAPACITY: usize = 4096;

pub fn run(config: Config) -> RunResult {
    use RuntimeJob::*;

    let mut errors = vec![];

    let regex_build_result = RegexBuilder
    ::new(config.search_expr.as_str())
        .case_insensitive(config.case_insensitive)
        .multi_line(config.multiline)
        .dot_matches_new_line(config.dot_matches_new_line)
        .swap_greed(config.swap_greed)
        .ignore_whitespace(config.ignore_whitespace_and_comments)
        .unicode(config.unicode)
        .build();

    let regex = match regex_build_result {
        Ok(regex) => regex,
        Err(e) => return Err(vec![format!("Regex parsing error: {e}")])
    };

    let runtime = Arc::new(Runtime {
        regex,
        config,
        results: Mutex::new(vec![]),
        jobs: JobsChan::<RuntimeJob>::with_capacity(DEFAULT_CHAN_CAPACITY),
    });

    for source in &runtime.config.sources {
        runtime.jobs.announce_one(Iterate(Arc::clone(source)));
    }

    let mut workers = vec![];
    for _ in 0..runtime.config.threads {
        let runtime = Arc::clone(&runtime);
        workers.push(std::thread::spawn(move || go_search_worker(runtime)))
    }

    runtime.jobs.wait_for_all_done();
    for worker in workers {
        worker.join().unwrap().unwrap_or_else(|e| errors.push(e));
    }

    if errors.len() != 0 {
        Err(errors)
    } else {
        Ok(())
    }
}

fn go_search_worker(shared_state: Arc<Runtime>) -> Result<(), RuntimeError> {
    use RuntimeJob::*;
    todo!()
}