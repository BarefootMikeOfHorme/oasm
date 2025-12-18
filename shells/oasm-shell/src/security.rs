use std::collections::HashSet;
use std::sync::Mutex;

/// Global capability registry (thread-safe)
static CAPABILITIES: Mutex<Option<HashSet<String>>> = Mutex::new(None);

/// Initialize the capability system with safe defaults
pub fn init_capabilities() {
    let mut caps = CAPABILITIES.lock().unwrap();
    let mut set = HashSet::new();

    // Start with minimal capabilities (principle of least privilege)
    set.insert("file_access".to_string());  // Read-only by default

    *caps = Some(set);
    println!("[SECURITY] Capabilities initialized (minimal set)");
}

/// Check if a capability is currently enabled
pub fn check_capability(cap: &str) -> bool {
    let caps = CAPABILITIES.lock().unwrap();
    if let Some(ref set) = *caps {
        set.contains(cap)
    } else {
        false
    }
}

/// Enable a capability (with user confirmation in production)
pub fn enable_capability(cap: &str) {
    let mut caps = CAPABILITIES.lock().unwrap();
    if let Some(ref mut set) = *caps {
        if set.insert(cap.to_string()) {
            println!("[SECURITY] Enabled capability: {}", cap);
            println!("[WARNING] This grants elevated permissions");
        } else {
            println!("[INFO] Capability '{}' already enabled", cap);
        }
    }
}

/// Disable a capability
pub fn disable_capability(cap: &str) {
    let mut caps = CAPABILITIES.lock().unwrap();
    if let Some(ref mut set) = *caps {
        if set.remove(cap) {
            println!("[SECURITY] Disabled capability: {}", cap);
        } else {
            println!("[INFO] Capability '{}' was not enabled", cap);
        }
    }
}

/// Get count of active capabilities (for status display)
pub fn get_active_caps() -> usize {
    let caps = CAPABILITIES.lock().unwrap();
    if let Some(ref set) = *caps {
        set.len()
    } else {
        0
    }
}

/// List all active capabilities
pub fn list_capabilities() {
    let caps = CAPABILITIES.lock().unwrap();
    if let Some(ref set) = *caps {
        println!("\nActive Capabilities:");
        for cap in set.iter() {
            println!("  - {}", cap);
        }
    } else {
        println!("Capability system not initialized");
    }
}
