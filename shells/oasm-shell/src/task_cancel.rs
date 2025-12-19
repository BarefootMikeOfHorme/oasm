#![allow(dead_code)]
//! Task cancellation hooks
pub fn cancel_task(id: u64) {
    println!("Task {} cancelled", id);
}
