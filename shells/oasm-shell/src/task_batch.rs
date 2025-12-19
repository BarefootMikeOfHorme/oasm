#![allow(dead_code)]
//! Batch execution manager
pub struct TaskBatch;

impl TaskBatch {
    pub fn run_batch<F>(tasks: Vec<F>)
    where
        F: FnOnce() + Send + 'static,
    {
        for task in tasks {
            std::thread::spawn(move || {
                task();
            });
        }
    }
}
