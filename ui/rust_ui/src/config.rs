use anyhow::{Result, Context, anyhow};
use serde::Deserialize;
use std::fs;
use tracing::{info, warn, error};

/// Lifecycle phases of the application
#[derive(Debug, Deserialize, Clone)]
pub enum Lifecycle {
    Loading,
    Running,
    Updating,
    CleaningUp,
    ShuttingDown,
}

/// Per‑module state (DLLs, plugins, IPC bridges, etc.)
#[derive(Debug, Deserialize, Clone)]
pub struct ModuleState {
    pub name: String,
    pub enabled: bool,
    pub validated: bool,
}

/// Central runtime state loaded from YAML configs
#[derive(Debug, Deserialize, Clone)]
pub struct AppState {
    pub profile_name: String,
    pub version: Option<String>,
    pub environment: Option<String>,
    pub lifecycle: Lifecycle,
    pub modules: Vec<ModuleState>,
}

/// Load the runtime state from YAML files
pub fn load_state() -> Result<AppState> {
    let runtime_path = "config/runtime.yaml";
    let default_path = "config/oasm.default.yaml";

    let yaml_str = match fs::read_to_string(runtime_path) {
        Ok(content) => {
            info!(path = %runtime_path, "Loaded runtime config");
            content
        }
        Err(e) => {
            warn!(path = %runtime_path, error = %e, "Could not read runtime config, falling back");
            fs::read_to_string(default_path)
                .with_context(|| format!("Failed to read fallback config at {}", default_path))?
        }
    };

    let state: AppState = serde_yaml::from_str(&yaml_str)
        .map_err(|e| {
            error!(error = %e, "YAML parsing error");
            anyhow!("Invalid YAML configuration: {}", e)
        })?;

    info!(profile = %state.profile_name, env = ?state.environment, "Config loaded successfully");
    Ok(state)
}

/// Validate the loaded state against schema rules
pub fn validate_state(state: &AppState) -> Result<()> {
    if state.profile_name.trim().is_empty() {
        error!("Profile name is missing in configuration");
        return Err(anyhow!("Profile name cannot be empty"));
    }

    if let Some(env) = &state.environment {
        if !["dev", "test", "prod"].contains(&env.as_str()) {
            warn!(env = %env, "Unexpected environment value");
        }
    } else {
        warn!("No environment specified, defaulting to 'dev'");
    }

    if state.modules.is_empty() {
        warn!("No modules defined in configuration");
    } else {
        for m in &state.modules {
            info!(module = %m.name, enabled = m.enabled, validated = m.validated, "Module state");
            if m.enabled && !m.validated {
                warn!(module = %m.name, "Module enabled but not validated");
            }
        }
    }

    info!("State validation passed.");
    Ok(())
}

/// Update lifecycle with logging
pub fn set_lifecycle(state: &mut AppState, phase: Lifecycle) {
    state.lifecycle = phase.clone();
    info!(phase = ?phase, "Lifecycle updated");
}
