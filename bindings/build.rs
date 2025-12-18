fn main() {
    println!(\"cargo:rerun-if-changed=native_lib.c\");
    cc::Build::new()
        .file(\"native_lib.c\")
        .compile(\"oasm_native\");
}

// Full course meal ideas:
// - Conditional builds for Windows/Linux/macOS
// - Link against system libraries
// - Generate bindings via bindgen
