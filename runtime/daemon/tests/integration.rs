#[test]
fn test_manifest_pipeline() {
    env_logger::try_init().ok();
    let path = "../../manifests/oasm_manifest.yaml";
    let result = compiler::compile_manifest(path);
    assert!(result.is_ok(), "Pipeline failed: {:?}", result);

    // Verify lineage log was updated
    let log_contents = std::fs::read_to_string("../lineage/lineage.log")
        .expect("Failed to read lineage.log");
    assert!(log_contents.contains("Manifest compiled successfully"),
            "Lineage log missing success entry");
}
