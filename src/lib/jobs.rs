use std::{
    sync::{
        Mutex,
        Condvar,
    }
};

struct JobsData<T>
where T: Sized {
    data: Vec<T>,
    started: usize,
    done: usize,
}

/// Chan for sync worker threads which are consumers and producers of jobs at the same time
pub struct JobsChan<T>
where T: Sized {
    jobs: Mutex<JobsData<T>>,
    job_announce: Condvar,
    done_jobs: Condvar,
}

impl<T> JobsChan<T>
where T: Sized {

    // new job can be announced (jobs.data.push(job)) by worker thread between wait_for_job_announcement (jobs.started++) and done_one (jobs.done++) calls
    // then other worker thread will grab announced job and do it, and it can announce more jobs during working on that job
    // so, in that way jobs.started is always +1 greater if at least one worker is doing a task
    // and jobs.started == jobs.finished otherwise

    pub fn with_capacity(buffer_size: usize) -> JobsChan<T> {
        JobsChan {
            jobs: Mutex::new(JobsData {
                data: Vec::<T>::with_capacity(buffer_size),
                started: 0,
                done: 0,
            }),
            job_announce: Condvar::new(),
            done_jobs: Condvar::new()
        }
    }

    pub fn wait_for_job_announcement(&self) -> Option<T> {
        let mut jobs = self.jobs.lock().unwrap();
        while jobs.data.len() == 0 && jobs.started != jobs.done {
            jobs = self.job_announce.wait(jobs).unwrap();
        }

        if jobs.started == jobs.done {
            None // no jobs had been announced and all jobs was done
        } else {
            jobs.started = jobs.started.wrapping_add(1);
            let job = jobs.data.pop().unwrap();
            Some(job)
        }
    }

    pub fn announce_one(&mut self, job: T) {
        let mut jobs = self.jobs.lock().unwrap();
        jobs.data.push(job);
        self.job_announce.notify_one();
    }

    pub fn done_one(&mut self) {
        let mut jobs = self.jobs.lock().unwrap();
        jobs.done += 1;
        if jobs.started == jobs.done {
            self.job_announce.notify_all();
            self.done_jobs.notify_all();
        }
    }

    pub fn wait_for_all_done(&self) {
        let mut jobs = self.jobs.lock().unwrap();
        while jobs.data.len() != 0 || jobs.started != jobs.done {
            jobs = self.done_jobs.wait(jobs).unwrap();
        }
    }
}