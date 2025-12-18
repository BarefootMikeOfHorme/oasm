mod ffi;

/// Safe wrapper around native FFI addition function.
///
/// # Safety
/// This function calls an unsafe extern "C" function but is safe because:
/// - Input parameters are Copy types (i32) with no memory concerns
/// - No pointers or references are passed
/// - Return value is a simple i32
pub fn add_numbers(a: i32, b: i32) -> i32 {
    // SAFETY: oasm_add takes two i32 values by value and returns i32.
    // No memory safety concerns exist for this simple arithmetic operation.
    unsafe { ffi::oasm_add(a, b) }
}

// Full course meal ideas:
// - Wrap multiple native functions
// - Error handling + Result types
// - Structured CBOR serialization across FFI boundary
