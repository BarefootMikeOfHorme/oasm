use anyhow::Result;
use notify::{Config, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use tokio::sync::mpsc::Sender;
use std::path::PathBuf;
use crate::types::WatchEvent;

/// Starts a file watcher for provided paths and emits WatchEvent into tx.
pub async fn start_watch(paths: Vec<String>, tx: Sender<WatchEvent>) -> Result<()> {
    let path_count = paths.len();
    let tx_clone = tx.clone();

    tokio::task::spawn_blocking(move || {
        let mut watcher = RecommendedWatcher::new(
            move |res: Result<notify::Event, notify::Error>| {
                match res {
                    Ok(event) => {
                        let path = event.paths.get(0).cloned().unwrap_or(PathBuf::from(""));
                        let path_str = path.to_string_lossy().to_string();
                        let ev = match event.kind {
                            EventKind::Create(_) => WatchEvent::Created { path: path_str },
                            EventKind::Modify(_) => WatchEvent::Changed { path: path_str },
                            EventKind::Remove(_) => WatchEvent::Removed { path: path_str },
                            _ => WatchEvent::Changed { path: path_str },
                        };
                        let _ = tx.blocking_send(ev);
                    }
                    Err(e) => {
                        let _ = tx.blocking_send(WatchEvent::Error {
                            message: format!("watch_error: {}", e),
                        });
                    }
                }
            },
            Config::default(),
        ).expect("Failed to create watcher");

        for p in &paths {
            if let Err(e) = watcher.watch(&PathBuf::from(p), RecursiveMode::NonRecursive) {
                let _ = tx_clone.blocking_send(WatchEvent::Error {
                    message: format!("watch_add_error path={} err={}", p, e),
                });
            }
        }

        // Keep watcher alive - it needs to run indefinitely
        loop {
            std::thread::sleep(std::time::Duration::from_secs(3600));
        }
    });

    log::info!("Watcher started for {} path(s)", path_count);
    Ok(())
}
