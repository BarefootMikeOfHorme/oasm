#![allow(dead_code)]
//! Task tracking system
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Task {
    pub id: u64,
    pub description: String,
    pub status: String,
}

pub struct TaskTracker {
    tasks: HashMap<u64, Task>,
    next_id: u64,
}

impl TaskTracker {
    pub fn new() -> Self {
        Self { tasks: HashMap::new(), next_id: 1 }
    }

    pub fn add_task(&mut self, description: &str) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        self.tasks.insert(id, Task { id, description: description.to_string(), status: "Pending".into() });
        id
    }

    pub fn update_status(&mut self, id: u64, status: &str) {
        if let Some(task) = self.tasks.get_mut(&id) {
            task.status = status.to_string();
        }
    }

    pub fn list_tasks(&self) -> Vec<&Task> {
        self.tasks.values().collect()
    }
}
