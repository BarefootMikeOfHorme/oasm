#![allow(dead_code)]
//! Task scheduling routines
use std::time::Duration;
use std::thread;

pub struct TaskScheduler;

impl TaskScheduler {
    pub fn schedule<F>(delay: Duration, task: F)
    where
        F: FnOnce() + Send + 'static,
    {
        thread::spawn(move || {
            thread::sleep(delay);
            task();
        });
    }

    pub fn schedule_repeating<F>(interval: Duration, mut task: F)
    where
        F: FnMut() + Send + 'static,
    {
        thread::spawn(move || {
            loop {
                thread::sleep(interval);
                task();
            }
        });
    }
}
