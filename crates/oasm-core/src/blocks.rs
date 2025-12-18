/// Block system for program-specific functionality

use crate::{Block, BlockRegistry};

/// Load blocks for a specific program type
pub fn load_blocks_for_program(program_type: &str) -> Vec<Block> {
    match program_type {
        "cad" => vec![
            Block {
                id: "cad_primitives_block".to_string(),
                program_type: "cad".to_string(),
                instructions: vec![
                    "CREATE".to_string(),
                    "EXTRUDE".to_string(),
                    "FILLET".to_string(),
                    "CHAMFER".to_string(),
                ],
                rules: vec![
                    "geometric_validation".to_string(),
                    "parametric_constraints".to_string(),
                ],
                optimizations: vec![
                    "parallel_primitive_creation".to_string(),
                    "batch_operations".to_string(),
                ],
            },
            Block {
                id: "cad_export_block".to_string(),
                program_type: "cad".to_string(),
                instructions: vec![
                    "EXPORT".to_string(),
                    "VALIDATE".to_string(),
                ],
                rules: vec![
                    "export_compatibility".to_string(),
                ],
                optimizations: vec![
                    "compression".to_string(),
                ],
            },
        ],
        "engine" => vec![
            Block {
                id: "engine_scene_block".to_string(),
                program_type: "engine".to_string(),
                instructions: vec![
                    "CREATE".to_string(),
                    "ATTACH".to_string(),
                    "SET".to_string(),
                ],
                rules: vec![
                    "scene_graph_validation".to_string(),
                    "physics_constraints".to_string(),
                ],
                optimizations: vec![
                    "entity_pooling".to_string(),
                    "batch_rendering".to_string(),
                ],
            },
        ],
        "document" => vec![
            Block {
                id: "document_content_block".to_string(),
                program_type: "document".to_string(),
                instructions: vec![
                    "INSERT".to_string(),
                    "APPLY".to_string(),
                    "EXPORT".to_string(),
                ],
                rules: vec![
                    "structure_validation".to_string(),
                    "style_consistency".to_string(),
                ],
                optimizations: vec![
                    "incremental_rendering".to_string(),
                ],
            },
        ],
        _ => Vec::new(),
    }
}

/// Initialize block registry with all program types
pub fn init_block_registry(registry: &mut BlockRegistry) {
    for program_type in &["cad", "engine", "document", "compression", "debug"] {
        for block in load_blocks_for_program(program_type) {
            registry.register(block);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_cad_blocks() {
        let blocks = load_blocks_for_program("cad");
        assert!(!blocks.is_empty());
        assert!(blocks.iter().any(|b| b.id == "cad_primitives_block"));
    }

    #[test]
    fn test_init_block_registry() {
        let mut registry = BlockRegistry::new();
        init_block_registry(&mut registry);
        let cad_blocks = registry.get_for_program("cad");
        assert!(!cad_blocks.is_empty());
    }
}
