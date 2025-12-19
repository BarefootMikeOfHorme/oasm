// Library interface for runtime_daemon
// Exposes core functionality for use by other crates (e.g., compiler)

pub mod types;
pub mod parser;
pub mod validator;
pub mod commit;
pub mod lineage;
pub mod converter;
pub mod handler;
pub mod manifest_loader;

// Re-export commonly used types and functions
pub use parser::{parse_manifest, to_yaml};
pub use validator::validate_manifest;
pub use commit::commit_text;
pub use lineage::record_event;
