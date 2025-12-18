use runtime_daemon::parser::{parse_manifest, to_yaml};
use runtime_daemon::validator::validate_manifest;
use runtime_daemon::commit::commit_text;
use runtime_daemon::lineage::record_event;

pub mod scanner;

pub fn compile_manifest(path: &str) -> Result<(), String> {
    log::info!("Compiler invoked on manifest: {}", path);

    let manifest = parse_manifest(path).map_err(|e| {
        log::error!("Parsing failed: {}", e);
        format!("{}", e)
    })?;

    let validated = validate_manifest(&manifest).map_err(|e| {
        log::error!("Validation failed: {}", e);
        format!("{}", e)
    })?;

    // Convert validated manifest back to YAML
    let yaml_contents = to_yaml(&validated).map_err(|e| {
        log::error!("YAML serialization failed: {}", e);
        format!("{}", e)
    })?;

    // Commit the validated YAML
    commit_text(path, &yaml_contents).map_err(|e| {
        log::error!("Commit failed: {}", e);
        format!("{}", e)
    })?;

    record_event(&format!("Manifest compiled successfully: {}", path)).ok();
    Ok(())
}
