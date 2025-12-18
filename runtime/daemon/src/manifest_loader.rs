/// Manifest Loader
/// Provides easy location and loading of modules, files, schemas, and settings
/// based on the master manifest

use std::path::{Path, PathBuf};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use anyhow::{Result, Context};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MasterManifest {
    pub manifest_version: String,
    pub oasm_version: String,
    pub last_updated: String,

    pub serialization: SerializationFormats,
    pub modules: Vec<ModuleInfo>,
    pub configs: ConfigLocations,
    pub schemas: Vec<SchemaInfo>,
    pub templates: TemplateLibrary,
    pub outputs: OutputLocations,
    pub integrations: Integrations,
    pub capabilities: Capabilities,
    pub load_order: LoadOrder,
    pub health: HealthMonitoring,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SerializationFormats {
    pub oasm: OasmFormats,
    pub objex: ObjexFormats,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct OasmFormats {
    pub primary: String,
    pub mirror: String,
    pub logs: String,
    pub schemas: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ObjexFormats {
    pub archive: String,
    pub runtime: String,
    pub exports: Vec<String>,
    pub metadata: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ModuleInfo {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub module_type: String,
    pub location: String,
    pub entry: Option<String>,
    pub config: Option<String>,
    pub schema: Option<String>,
    pub manifest: Option<String>,
    pub dlls: Option<Vec<String>>,
    pub capabilities: Vec<String>,
    pub auto_start: bool,
    pub dependencies: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ConfigLocations {
    pub runtime: ConfigFile,
    pub ui: ConfigFile,
    pub daemon: ConfigFile,
    pub shell: ConfigFile,
    pub compiler: ConfigFile,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ConfigFile {
    pub primary: String,
    pub schema: Option<String>,
    pub fallback: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SchemaInfo {
    pub id: String,
    pub format: String,
    pub location: String,
    pub validates: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TemplateLibrary {
    pub schemas: TemplateCategory,
    pub scripts: TemplateCategory,
    pub commands: TemplateCategory,
    pub workflows: TemplateCategory,
    pub scans: TemplateCategory,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TemplateCategory {
    pub location: String,
    pub index: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct OutputLocations {
    pub logs: LogLocations,
    pub exports: ExportLocations,
    pub cache: CacheLocations,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LogLocations {
    pub structure_debug: String,
    pub daemon_logs: String,
    pub lineage: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ExportLocations {
    pub cad: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CacheLocations {
    pub build: String,
    pub temp: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Integrations {
    pub powershell: PowerShellIntegration,
    pub python: PythonIntegration,
    pub wpshell: WpShellIntegration,
    pub objex: ObjexIntegration,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PowerShellIntegration {
    pub module: String,
    pub scripts: String,
    pub entry: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PythonIntegration {
    pub plugins: String,
    pub venv: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WpShellIntegration {
    pub enabled: bool,
    pub profile: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ObjexIntegration {
    pub enabled: bool,
    pub hdf5_archives: Option<String>,
    pub primitives: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Capabilities {
    pub available: Vec<String>,
    pub default_enabled: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LoadOrder {
    pub bootstrap: Vec<String>,
    pub startup: Vec<String>,
    pub on_demand: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct HealthMonitoring {
    pub heartbeat_file: String,
    pub daemon_status: String,
    pub context_status: String,
    pub checks: Vec<HealthCheck>,
    pub alerts: HashMap<String, String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct HealthCheck {
    pub module: String,
    pub interval: String,
}

/// Manifest Loader - Easy access to all OASM components
pub struct ManifestLoader {
    manifest: MasterManifest,
    root: PathBuf,
}

impl ManifestLoader {
    /// Load the master manifest from a file
    pub fn load(manifest_path: impl AsRef<Path>) -> Result<Self> {
        let content = std::fs::read_to_string(&manifest_path)
            .context("Failed to read manifest file")?;

        let manifest: MasterManifest = serde_yaml::from_str(&content)
            .context("Failed to parse manifest YAML")?;

        let root = manifest_path.as_ref()
            .parent()
            .and_then(|p| p.parent())
            .context("Invalid manifest path")?
            .to_path_buf();

        Ok(Self { manifest, root })
    }

    /// Get module by ID
    pub fn get_module(&self, id: &str) -> Option<&ModuleInfo> {
        self.manifest.modules.iter().find(|m| m.id == id)
    }

    /// Get absolute path for a module
    pub fn module_path(&self, id: &str) -> Option<PathBuf> {
        self.get_module(id)
            .map(|m| self.root.join(&m.location))
    }

    /// Get module entry point (executable)
    pub fn module_entry(&self, id: &str) -> Option<PathBuf> {
        self.get_module(id)
            .and_then(|m| m.entry.as_ref())
            .map(|e| self.root.join(e))
    }

    /// Get config file path
    pub fn config_path(&self, config_type: &str) -> Option<PathBuf> {
        match config_type {
            "runtime" => Some(self.root.join(&self.manifest.configs.runtime.primary)),
            "ui" => Some(self.root.join(&self.manifest.configs.ui.primary)),
            "daemon" => Some(self.root.join(&self.manifest.configs.daemon.primary)),
            "shell" => Some(self.root.join(&self.manifest.configs.shell.primary)),
            "compiler" => Some(self.root.join(&self.manifest.configs.compiler.primary)),
            _ => None,
        }
    }

    /// Get schema by ID
    pub fn get_schema(&self, id: &str) -> Option<&SchemaInfo> {
        self.manifest.schemas.iter().find(|s| s.id == id)
    }

    /// Get schema file path
    pub fn schema_path(&self, id: &str) -> Option<PathBuf> {
        self.get_schema(id)
            .map(|s| self.root.join(&s.location))
    }

    /// Get template path
    pub fn template_path(&self, category: &str, name: &str) -> Option<PathBuf> {
        let category_info = match category {
            "schemas" => &self.manifest.templates.schemas,
            "scripts" => &self.manifest.templates.scripts,
            "commands" => &self.manifest.templates.commands,
            "workflows" => &self.manifest.templates.workflows,
            "scans" => &self.manifest.templates.scans,
            _ => return None,
        };

        Some(self.root.join(&category_info.location).join(name))
    }

    /// Get all modules that should auto-start
    pub fn auto_start_modules(&self) -> Vec<&ModuleInfo> {
        self.manifest.modules.iter()
            .filter(|m| m.auto_start)
            .collect()
    }

    /// Get modules in load order
    pub fn load_order(&self) -> (&Vec<String>, &Vec<String>, &Vec<String>) {
        (
            &self.manifest.load_order.bootstrap,
            &self.manifest.load_order.startup,
            &self.manifest.load_order.on_demand,
        )
    }

    /// Check if a capability is available
    pub fn has_capability(&self, cap: &str) -> bool {
        self.manifest.capabilities.available.contains(&cap.to_string())
    }

    /// Get default enabled capabilities
    pub fn default_capabilities(&self) -> &Vec<String> {
        &self.manifest.capabilities.default_enabled
    }

    /// Get output path
    pub fn output_path(&self, output_type: &str) -> Option<PathBuf> {
        match output_type {
            "structure_debug" => Some(self.root.join(&self.manifest.outputs.logs.structure_debug)),
            "daemon_logs" => Some(self.root.join(&self.manifest.outputs.logs.daemon_logs)),
            "lineage" => Some(self.root.join(&self.manifest.outputs.logs.lineage)),
            "cad_exports" => Some(self.root.join(&self.manifest.outputs.exports.cad)),
            "build_cache" => Some(self.root.join(&self.manifest.outputs.cache.build)),
            "temp" => Some(self.root.join(&self.manifest.outputs.cache.temp)),
            _ => None,
        }
    }

    /// Get integration info
    pub fn integration_enabled(&self, integration: &str) -> bool {
        match integration {
            "wpshell" => self.manifest.integrations.wpshell.enabled,
            "objex" => self.manifest.integrations.objex.enabled,
            _ => false,
        }
    }

    /// Get the full manifest
    pub fn manifest(&self) -> &MasterManifest {
        &self.manifest
    }

    /// Get root directory
    pub fn root(&self) -> &Path {
        &self.root
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manifest_loading() {
        // This would load the actual manifest in a real test
        // For now, just a placeholder
    }
}
