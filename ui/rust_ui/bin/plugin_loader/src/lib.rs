use libloading::{Library, Symbol};
use tracing::{info, error};

#[no_mangle]
/// # Safety
/// Caller must ensure inputs are valid and safe to use.
pub unsafe extern "C" fn load_plug_fn(path_ptr: *const u8, len: usize) -> bool {
    let slice = std::slice::from_raw_parts(path_ptr, len);
    let path = String::from_utf8_lossy(slice).to_string();

    match Library::new(&path) {
        Ok(lib) => {
            info!("load_plug_fn: loaded {}", path);
            match lib.get::<Symbol<unsafe extern "C" fn()>>(b"plugin_entry") {
                Ok(_) => info!("plugin_entry symbol present"),
                Err(_) => info!("plugin_entry symbol not found"),
            }
            true
        }
        Err(e) => {
            error!("load_plug_fn: failed to load {}: {}", path, e);
            false
        }
    }
}
