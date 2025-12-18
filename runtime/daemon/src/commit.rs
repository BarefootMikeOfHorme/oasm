use anyhow::Result;
use std::fs::OpenOptions;
use std::io::Write;
use tempfile::NamedTempFile;

pub fn commit_text(path: &str, contents: &str) -> Result<()> {
    let mut tmp = NamedTempFile::new()?;
    tmp.write_all(contents.as_bytes())?;
    let _ = tmp.persist(path)?;
    log::info!("Committed file: {}", path);
    Ok(())
}

pub fn append_line(path: &str, line: &str) -> Result<()> {
    let mut f = OpenOptions::new().create(true).append(true).open(path)?;
    writeln!(f, "{}", line)?;
    log::info!("Appended line to {}: {}", path, line);
    Ok(())
}
