mod daemon;
mod commit;
mod converter;
mod handler;
mod lineage;
mod parser;
mod supervisor;
mod types;
mod validator;
mod watch;
pub mod manifest_loader;


fn main() {
    env_logger::init();
    log::info!("Daemon starting up");

    if let Err(e) = ctrlc::set_handler(|| {
        log::warn!("Shutdown signal received, terminating daemon...");
        std::process::exit(0);
    }) {
        log::error!("Failed to set Ctrl-C handler: {}", e);
    }

    let watch_paths = vec![
        "crate_manifest.yaml".to_string(),
        "compiler/compiler.yaml".to_string(),
        "runtime/daemon/daemon.yaml".to_string(),
        "ui/rust_ui/rust_ui.yaml".to_string(),
    ];

    match crate::daemon::Daemon::new(watch_paths).start() {
        Ok(_) => {
            log::info!("Supervisor loop exited cleanly");
            std::process::exit(0);
        }
        Err(e) => {
            log::error!("Supervisor loop failed: {}", e);
            std::process::exit(1);
        }
    }
}
