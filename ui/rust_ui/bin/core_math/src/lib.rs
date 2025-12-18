use tracing::{info, warn};

#[no_mangle]
/// # Safety
/// Caller must ensure inputs are valid and safe to use.
pub unsafe extern "C" fn add_i32_core(a: i32, b: i32) -> i32 {
    let r = a + b;
    info!("add_i32_core: {} + {} = {}", a, b, r);
    r
}

#[no_mangle]
/// # Safety
/// Caller must ensure inputs are valid and safe to use.
pub unsafe extern "C" fn mul_i32_core(a: i32, b: i32) -> i32 {
    let r = a * b;
    info!("mul_i32_core: {} * {} = {}", a, b, r);
    r
}

#[no_mangle]
/// # Safety
/// Caller must ensure inputs are valid and safe to use.
pub unsafe extern "C" fn div_i32_core(a: i32, b: i32) -> i32 {
    if b == 0 {
        warn!("div_i32_core divide by zero: {} / {}", a, b);
        0
    } else {
        let r = a / b;
        info!("div_i32_core: {} / {} = {}", a, b, r);
        r
    }
}
