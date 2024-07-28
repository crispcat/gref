use std::{
    io::Read,
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
    lib::config::{
        Config,
        TextSource
    },
    lib::jobs::JobsChan,
};

pub struct SearchResult {
    line : usize,
    position : usize,
    needle : String,
    haystack : TextSource
}

enum Jobs {
    ParseStdin,
    ParseDir(String),
    ParseFile(String),
    SearchString(String)
}

struct SharedState {
    regex: Regex,
    jobs : JobsChan<Jobs>,
    results : Mutex<Vec<SearchResult>>
}

type RuntimeError = String;
type RunResult = Result<(), Vec<RuntimeError>>;

const DEFAULT_CHAN_CAPACITY: usize = 4096;

pub fn run(config: Config) -> RunResult {

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

    let shared_state = Arc::new(SharedState {
        regex,
        results: Mutex::new(vec![]),
        jobs: JobsChan::<Jobs>::with_capacity(DEFAULT_CHAN_CAPACITY),
    });

    // todo: schedule initial jobs

    let mut workers = vec![];

    for t in 0..config.threads {
        let shared_state = Arc::clone(&shared_state);
        workers.push(std::thread::spawn(move || go_worker(shared_state)))
    }

    // todo: collect results and write throw mpsc chan
    // todo: or return runtime object with chan and handles to wait threads

    shared_state.jobs.wait_for_all_done();
    for worker in workers {
        worker.join().unwrap().unwrap_or_else(|e| errors.push(e));
    }

    if errors.len() != 0 {
        Err(errors)
    } else {
        Ok(())
    }
}

fn go_worker(shared_state: Arc<SharedState>) -> Result<(), RuntimeError> {
    use Jobs::*;
    todo!()
}