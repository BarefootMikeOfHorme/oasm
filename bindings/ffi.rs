// Bare-bones FFI declarations
extern \"C\" {
    pub fn oasm_add(a: i32, b: i32) -> i32;
}

// Full course meal ideas:
// - Expand to IPC, task orchestration, sandbox calls
// - Safe Rust wrappers around C APIs
// - Capability-gated FFI surface
