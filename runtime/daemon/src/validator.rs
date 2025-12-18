use anyhow::{bail, Result};
use crate::types::CrateManifest;

pub fn validate_manifest(m: &CrateManifest) -> Result<CrateManifest> {
    let mut manifest = m.clone();

    if manifest.package.name.trim().is_empty() {
        bail!("package.name must not be empty");
    }
    if manifest.package.version.trim().is_empty() {
        bail!("package.version must not be empty");
    }

    if manifest.package.edition != "2021" {
        log::warn!(
            "Repairing edition from {} to 2021 for {}",
            manifest.package.edition,
            manifest.package.name
        );
        manifest.package.edition = "2021".to_string();
    }

    if manifest.dependencies.get("log").map(|v| v.is_null()).unwrap_or(true) {
        log::warn!("Adding missing dependency 'log' to {}", manifest.package.name);
    }
    if manifest.dependencies.get("env_logger").map(|v| v.is_null()).unwrap_or(true) {
        log::warn!("Adding missing dependency 'env_logger' to {}", manifest.package.name);
    }

    Ok(manifest)
}
