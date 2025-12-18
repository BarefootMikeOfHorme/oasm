use anyhow::Result;

pub struct Daemon {
    pub watch_paths: Vec<String>,
}

impl Daemon {
    pub fn new(watch_paths: Vec<String>) -> Self {
        Self { watch_paths }
    }

    pub fn start(&self) -> Result<()> {
        log::info!("Daemon starting with {} path(s)", self.watch_paths.len());
        crate::supervisor::run(&self.watch_paths)
    }
}
