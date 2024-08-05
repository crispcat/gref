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

    // new job can be announced (jobs.data.push(job)) by worker thread between wait_for_job_announcement (jobs.started++) and done_one (jobs.done++) calls
    // then other worker thread will grab announced job and do it, and it can announce more jobs during working on that job
    // so, in that way jobs.started is always at least +1 greater if at least one worker is doing a task
    // and jobs.started == jobs.finished otherwise

    pub fn wait_for_one(&self) -> Option<JobHandler<T>> {
        let mut jobs = self.jobs.lock().unwrap();
        while jobs.data.len() == 0 && (jobs.started != jobs.done || jobs.started == 0) {
            jobs = self.job_announce.wait(jobs).unwrap();
        }
        if jobs.data.len() == 0 {
            // no new jobs was announced and all started jobs are done
            // worker thread must now finish gracefully
            None
        } else {
            // start a new job and move it into worker thread context
            jobs.started = jobs.started.wrapping_add(1);
            let job = jobs.data.pop().unwrap();
            Some(JobHandler::<T>(job, self))
        }
    }

    pub fn announce_one(&self, job: T) {
        let mut jobs = self.jobs.lock().unwrap();
        jobs.data.push(job);
        self.job_announce.notify_one();
    }

    pub fn done_one(&self) {
        let mut jobs = self.jobs.lock().unwrap();
        jobs.done = jobs.done.wrapping_add(1);
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

pub struct JobHandler<'a, T>(T, &'a JobsChan<T>)
where T: Sized;

impl<T> JobHandler<'_, T> {
    pub fn job(&self) -> &T {
        &self.0
    }
}

impl<T> Drop for JobHandler<'_, T> {
    fn drop(&mut self) {
        self.1.done_one()
    }
}