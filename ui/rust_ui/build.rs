use std::path::Path;
use std::process::Command;

fn main() {
    // Allow override via environment variable
    if let Ok(env_path) = std::env::var("PYO3_PYTHON") {
        println!("cargo:rustc-env=PYO3_PYTHON={}", env_path);
        println!("cargo:warning=Using Python from env override: {}", env_path);
        return;
    }

    // Hardcoded Python 3.12 path
    let py312 = r"C:\Users\Administrator\AppData\Local\Programs\Python\Python312\python.exe";
    if Path::new(py312).exists() {
        println!("cargo:rustc-env=PYO3_PYTHON={}", py312);
        println!("cargo:warning=Using Python 3.12 for PyO3 (full compatibility).");
        return;
    }

    // Search PATH for Python executables
    if let Ok(output) = Command::new("where").arg("python").output() {
        if output.status.success() {
            let paths = String::from_utf8_lossy(&output.stdout);
            for candidate in paths.lines() {
                if candidate.contains("Python312") || candidate.contains("Python311") || candidate.contains("Python310") {
                    println!("cargo:rustc-env=PYO3_PYTHON={}", candidate);
                    println!("cargo:warning=Found Python at {}", candidate);
                    return;
                }
            }
        }
    }

    // Fallback: ABI3 forward compatibility
    println!("cargo:rustc-env=PYO3_USE_ABI3_FORWARD_COMPATIBILITY=1");
    println!("cargo:warning=Python 3.12 not found. Enabling ABI3 forward compatibility.");
}
