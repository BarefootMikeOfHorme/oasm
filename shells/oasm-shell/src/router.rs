use crate::security;

/// Routes commands to appropriate handlers with capability checking.
/// Provides clear error messages and recovery suggestions (executive function support).
pub fn route(cmd: &str) {
    let parts: Vec<&str> = cmd.split_whitespace().collect();

    if parts.is_empty() {
        return;
    }

    let command = parts[0];
    let args = &parts[1..];

    match command {
        "run" | "exec" => {
            if !security::check_capability("process_control") {
                println!("ERROR: Process execution requires 'process_control' capability");
                println!("SUGGESTION: Enable capability with: enable process_control");
                return;
            }
            if args.is_empty() {
                println!("ERROR: Missing program name");
                println!("USAGE: run <program> [args...]");
                return;
            }
            execute_program(args);
        }
        "read" | "cat" => {
            if !security::check_capability("file_access") {
                println!("ERROR: File reading requires 'file_access' capability");
                return;
            }
            if args.is_empty() {
                println!("ERROR: Missing file path");
                println!("USAGE: read <filepath>");
                return;
            }
            read_file(args[0]);
        }
        "write" => {
            if !security::check_capability("file_access") {
                println!("ERROR: File writing requires 'file_access' capability");
                return;
            }
            if args.len() < 2 {
                println!("ERROR: Missing file path or content");
                println!("USAGE: write <filepath> <content>");
                return;
            }
            write_file(args[0], &args[1..].join(" "));
        }
        "ipc" => {
            if !security::check_capability("ipc") {
                println!("ERROR: IPC requires 'ipc' capability");
                return;
            }
            println!("IPC: Feature under development");
        }
        "enable" => {
            if args.is_empty() {
                println!("ERROR: Missing capability name");
                println!("AVAILABLE: file_access, process_control, ipc, network");
                return;
            }
            security::enable_capability(args[0]);
        }
        "disable" => {
            if args.is_empty() {
                println!("ERROR: Missing capability name");
                return;
            }
            security::disable_capability(args[0]);
        }
        _ => {
            println!("ERROR: Unknown command '{}'", command);
            println!("SUGGESTION: Type 'help' to see available commands");
        }
    }
}

fn execute_program(args: &[&str]) {
    println!("[EXEC] Would execute: {}", args.join(" "));
    println!("[INFO] Process execution will be implemented with job objects");
}

fn read_file(path: &str) {
    println!("[READ] Would read file: {}", path);
    println!("[INFO] File operations will use capability-gated I/O");
}

fn write_file(path: &str, content: &str) {
    println!("[WRITE] Would write to: {}", path);
    println!("[CONTENT] {}", content);
    println!("[INFO] Writes will be logged in lineage for audit trail");
}
