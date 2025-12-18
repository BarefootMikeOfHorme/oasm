use anyhow::Result;
use tokio::runtime::Builder;
use tokio::sync::mpsc;
use crate::types::WatchEvent;

/// Runs the supervisor loop: watches paths, processes events, records lineage.
pub fn run(paths: &[String]) -> Result<()> {
    let rt = Builder::new_multi_thread()
        .enable_all()
        .build()?;

    rt.block_on(async move {
        let (tx, mut rx) = mpsc::channel::<WatchEvent>(128);

        // Start watcher task
        crate::watch::start_watch(paths.to_vec(), tx).await?;

        // Initial scan: process manifests once
        initialize(paths).await;

        // Event loop
        while let Some(ev) = rx.recv().await {
            match &ev {
                WatchEvent::Created { path } | WatchEvent::Changed { path } => {
                    process_manifest(path).await;
                }
                WatchEvent::Removed { path } => {
                    crate::lineage::record_event(&format!("manifest_removed path={}", path)).ok();
                    crate::lineage::record_event_cbor("removed", path).ok();
                }
                WatchEvent::Error { message } => {
                    crate::handler::handle_error("watch", anyhow::anyhow!(message.clone()));
                }
            }
        }

        Ok::<(), anyhow::Error>(())
    })?;

    Ok(())
}

async fn initialize(paths: &[String]) {
    for p in paths {
        process_manifest(p).await;
    }
}

async fn process_manifest(path: &str) {
    use crate::{converter, handler, lineage, parser, validator};

    if let Some(manifest) = handler::with_context("parse", || parser::parse_manifest(path)) {
        if let Some(valid) = handler::with_context("validate", || validator::validate_manifest(&manifest)) {
            // Convert to CBOR
            let cbor_out = cbor_out_path(path);
            handler::with_context("convert_to_cbor", || converter::to_cbor_file(&cbor_out, &valid));

            // Optionally commit repaired YAML if validator changed fields
            if let Ok(yaml) = parser::to_yaml(&valid) {
                handler::with_context("commit_text", || crate::commit::commit_text(path, &yaml));
            }

            let msg = format!("manifest_processed path={} cbor={}", path, cbor_out);
            lineage::record_event(&msg).ok();
            lineage::record_event_cbor("processed", &msg).ok();
        }
    }
}

fn cbor_out_path(yaml_path: &str) -> String {
    let file_name = yaml_path
        .rsplit_once('/')
        .map(|(_, f)| f)
        .or_else(|| yaml_path.rsplit_once('\\').map(|(_, f)| f))
        .unwrap_or("manifest.cbor");
    format!("runtime/daemon/cbor/{}.cbor", file_name.trim_end_matches(".yaml"))
}
