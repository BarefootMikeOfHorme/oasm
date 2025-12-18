/// Module loading and management system

use std::path::{Path, PathBuf};
use crate::{Module, ModuleType, ModuleRegistry};

/// Load modules from a directory
pub fn load_from_directory(path: impl AsRef<Path>) -> Result<Vec<Module>, String> {
    let mut modules = Vec::new();
    // Implementation for loading modules
    Ok(modules)
}

/// Load a single module from a file
pub fn load_module(path: impl AsRef<Path>) -> Result<Module, String> {
    // Stub implementation
    Ok(Module {
        id: "stub".to_string(),
        name: "Stub Module".to_string(),
        module_type: ModuleType::Core,
        location: PathBuf::from(path.as_ref()),
        capabilities: Vec::new(),
    })
}

/// Initialize the module registry with core modules
pub fn init_core_modules(registry: &mut ModuleRegistry) {
    // CAD module
    registry.register(Module {
        id: "cad_primitives".to_string(),
        name: "CAD Primitives".to_string(),
        module_type: ModuleType::Core,
        location: PathBuf::from("modules/cad/primitives"),
        capabilities: vec![
            "create_cube".to_string(),
            "create_sphere".to_string(),
            "create_cylinder".to_string(),
            "extrude".to_string(),
        ],
    });

    // Engine module
    registry.register(Module {
        id: "engine_core".to_string(),
        name: "Engine Core".to_string(),
        module_type: ModuleType::Core,
        location: PathBuf::from("modules/engine/core"),
        capabilities: vec![
            "create_entity".to_string(),
            "add_component".to_string(),
            "scene_graph".to_string(),
        ],
    });

    // Document module
    registry.register(Module {
        id: "document_formatting".to_string(),
        name: "Document Formatting".to_string(),
        module_type: ModuleType::Core,
        location: PathBuf::from("modules/document/formatting"),
        capabilities: vec![
            "insert_heading".to_string(),
            "insert_paragraph".to_string(),
            "apply_style".to_string(),
        ],
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_core_modules() {
        let mut registry = ModuleRegistry::new();
        init_core_modules(&mut registry);
        assert!(registry.get("cad_primitives").is_some());
        assert!(registry.get("engine_core").is_some());
        assert!(registry.get("document_formatting").is_some());
    }
}
