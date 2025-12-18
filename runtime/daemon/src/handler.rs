use anyhow::Error;

pub fn handle_error(context: &str, err: Error) {
    log::error!("Error in {}: {:?}", context, err);
    let msg = format!("error context={} message={}", context, err);
    if let Err(e) = crate::lineage::record_event(&msg) {
        log::error!("Failed to record lineage error: {}", e);
    }
}

pub fn with_context<T, F>(context: &str, f: F) -> Option<T>
where
    F: FnOnce() -> anyhow::Result<T>,
{
    match f() {
        Ok(v) => Some(v),
        Err(e) => {
            handle_error(context, e);
            None
        }
    }
}
