use anyhow::Result;
use chrono::Utc;
use serde::Serialize;
use std::fs::OpenOptions;
use std::io::Write;   // <-- Added import

const LINEAGE_LOG: &str = "runtime/daemon/lineage/lineage.log";
const LINEAGE_CBOR: &str = "runtime/daemon/lineage/lineage.cbor";

pub fn record_event(line: &str) -> Result<()> {
    let ts = Utc::now().to_rfc3339();
    let entry = format!("{} {}", ts, line);
    OpenOptions::new()
        .create(true)
        .append(true)
        .open(LINEAGE_LOG)?
        .write_all(format!("{}\n", entry).as_bytes())?;
    Ok(())
}

#[derive(Serialize)]
struct Event<'a> {
    ts: String,
    kind: &'a str,
    msg: &'a str,
}

pub fn record_event_cbor(kind: &str, msg: &str) -> Result<()> {
    let ev = Event { ts: Utc::now().to_rfc3339(), kind, msg };
    let encoded = serde_cbor::to_vec(&ev)?;
    OpenOptions::new()
        .create(true)
        .append(true)
        .open(LINEAGE_CBOR)?
        .write_all(&encoded)?;
    Ok(())
}
