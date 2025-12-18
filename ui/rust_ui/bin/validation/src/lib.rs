use regex::Regex;
use tracing::info;

#[no_mangle]
/// # Safety
/// Caller must ensure inputs are valid and safe to use.
pub unsafe extern "C" fn val_email_fn(ptr: *const u8, len: usize) -> bool {
    let slice = std::slice::from_raw_parts(ptr, len);
    let email = String::from_utf8_lossy(slice);
    let re = Regex::new(r"^[^@\s]+@[^@\s]+\.[^@\s]+$").unwrap();
    let ok = re.is_match(&email);
    info!("val_email_fn: {} => {}", email, ok);
    ok
}
