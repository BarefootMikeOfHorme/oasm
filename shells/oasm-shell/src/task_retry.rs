#![allow(dead_code)]
//! Retry failed tasks logic
pub fn retry_task<F>(mut task: F, max_retries: u32)
where
    F: FnMut() -> bool,
{
    for attempt in 1..=max_retries {
        if task() {
            println!("Task succeeded on attempt {}", attempt);
            return;
        }
        println!("Attempt {} failed", attempt);
    }
    println!("Task failed after {} retries", max_retries);
}
