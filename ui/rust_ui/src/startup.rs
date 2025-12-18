use crate::config::AppState;
use anyhow::{Result, anyhow};
use std::env;
use tracing::{info, warn};

/// Initialize environment variables based on AppState configuration
pub fn init_environment(app_state: &mut AppState) -> Result<()> {
    // Set environment based on AppState
    let env_val = app_state.environment.as_deref().unwrap_or("dev");
    env::set_var("OASM_MODE", env_val);
    env::set_var("OASM_PROFILE", &app_state.profile_name);

    // Determine enabled features from modules
    let ui_enabled = app_state.modules.iter()
        .any(|m| m.name.contains("ui") && m.enabled);
    let cli_enabled = app_state.modules.iter()
        .any(|m| m.name.contains("cli") && m.enabled);
    let bindings_enabled = app_state.modules.iter()
        .any(|m| m.name.contains("bindings") && m.enabled);

    env::set_var("OASM_UI_ENABLED", ui_enabled.to_string());
    env::set_var("OASM_CLI_ENABLED", cli_enabled.to_string());
    env::set_var("OASM_BINDINGS_ENABLED", bindings_enabled.to_string());

    info!(
        "Environment initialized (mode={}, profile={})",
        env_val, app_state.profile_name
    );

    if !ui_enabled && !cli_enabled {
        warn!("Both UI and CLI are disabled — system will run headless.");
    }
    Ok(())
}

/// Monitor environment configuration validity
pub fn supervise_env() -> Result<()> {
    let mode = env::var("OASM_MODE").unwrap_or_else(|_| "unknown".into());
    match mode.as_str() {
        "production" | "staging" | "dev" => Ok(()),
        _ => Err(anyhow!("unexpected OASM_MODE value: {}", mode)),
    }
}
