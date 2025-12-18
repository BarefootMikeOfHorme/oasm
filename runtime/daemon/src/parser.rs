use anyhow::Result;
use std::fs;
use crate::types::CrateManifest;

pub fn parse_manifest(path: &str) -> Result<CrateManifest> {
    log::info!("Parsing manifest: {}", path);
    let contents = fs::read_to_string(path)?;
    let manifest: CrateManifest = serde_yaml::from_str(&contents)?;
    Ok(manifest)
}

pub fn to_yaml(manifest: &CrateManifest) -> Result<String> {
    Ok(serde_yaml::to_string(manifest)?)
}
