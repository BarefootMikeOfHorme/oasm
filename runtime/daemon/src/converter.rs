use anyhow::Result;
use serde::Serialize;

pub fn to_cbor_file<T: Serialize>(out_path: &str, data: &T) -> Result<()> {
    let encoded = serde_cbor::to_vec(data)?;
    std::fs::write(out_path, encoded)?;
    log::info!("CBOR written: {}", out_path);
    Ok(())
}
