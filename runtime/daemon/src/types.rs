use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub edition: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CrateManifest {
    pub package: Package,
    #[serde(default)]
    pub dependencies: serde_json::Value,
    #[serde(default, rename = "dev-dependencies")]
    pub dev_dependencies: serde_json::Value,
}

#[derive(Debug, Clone)]
pub enum WatchEvent {
    Created { path: String },
    Changed { path: String },
    Removed { path: String },
    Error { message: String },
}
