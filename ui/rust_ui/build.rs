use std::path::Path;
use std::process::Command;

fn main() {
    let py312 = r"C:\Users\Administrator\AppData\Local\Programs\Python\Python312\python.exe";
    if Path::new(py312).exists() {
        println!("cargo:rustc-env=PYO3_PYTHON={}", py312);
        println!("cargo:warning=Using Python 3.12 for PyO3 (full compatibility).");
        return;
    }
    if let Ok(output) = Command::new("where").arg("python").output() {
        if output.status.success() {
            let paths = String::from_utf8_lossy(&output.stdout);
            for candidate in paths.lines() {
                if candidate.contains("Python312") {
                    println!("cargo:rustc-env=PYO3_PYTHON={}", candidate);
                    println!("cargo:warning=Found Python 3.12 at {}", candidate);
                    return;
                }
            }
        }
    }
    println!("cargo:rustc-env=PYO3_USE_ABI3_FORWARD_COMPATIBILITY=1");
    println!("cargo:warning=Python 3.12 not found. Enabling ABI3 forward compatibility.");
}
