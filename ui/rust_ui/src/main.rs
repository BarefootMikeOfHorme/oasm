use tracing::{info, warn, error};
use tracing_subscriber::FmtSubscriber;
use tokio::time::{sleep, Duration};
use anyhow::Result;

mod validation;
mod startup;
mod config;

#[tokio::main]
async fn main() -> Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_target(false)
        .with_max_level(tracing::Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("set subscriber");

    info!("=== OASM Rust UI v0.1 Premium Handler ===");

    let mut state = config::load_state()?;
    info!("Loaded state profile: {}", state.profile_name);

    startup::init_environment(&mut state)?;

    validation::validate_runtime_config(&state).unwrap_or_else(|e| error!("Runtime config error: {e}"));
    validation::validate_bindings(&state).unwrap_or_else(|e| error!("Bindings validation error: {e}"));
    validation::validate_dlls(&state).unwrap_or_else(|e| error!("DLL validation error: {e}"));

    tokio::spawn(async {
        loop {
            if let Err(e) = startup::supervise_env() {
                warn!("Env supervision reported: {e}");
            }
            sleep(Duration::from_secs(10)).await;
        }
    });

    info!("Startup sequence complete. UI and CLI ready.");
    Ok(())
}
