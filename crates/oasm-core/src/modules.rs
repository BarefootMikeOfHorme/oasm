use std::path::Path;
use crate::Module;

pub fn load_from_directory(_path: impl AsRef<Path>) -> Result<Vec<Module>, String> {
    let modules = Vec::new(); // removed unnecessary mut
    Ok(modules)
}
