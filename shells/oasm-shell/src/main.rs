mod conpty;
mod router;
mod security;
mod python_bridge;

use std::io::{self, Write};

fn main() {
    println!("=== OASM Shell v0.1 - Executive Function Assistant ===");
    println!("Type 'help' for commands, 'exit' to quit\n");

    // Initialize security/capability system
    security::init_capabilities();

    // Command history for recall (executive function support)
    let mut history: Vec<String> = Vec::new();
    let mut task_count = 0u32;

    loop {
        // Clear, structured prompt (reduces cognitive load)
        print!("oasm[{}]> ", task_count);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let cmd = input.trim();

                // Empty command handling
                if cmd.is_empty() {
                    continue;
                }

                // Add to history (working memory support)
                history.push(cmd.to_string());
                task_count += 1;

                // Built-in commands
                match cmd {
                    "exit" | "quit" => {
                        println!("Tasks completed: {}", task_count - 1);
                        println!("Goodbye!");
                        break;
                    }
                    "history" => {
                        println!("\nCommand History:");
                        for (i, h) in history.iter().enumerate() {
                            println!("  {}: {}", i + 1, h);
                        }
                        println!();
                        continue;
                    }
                    "clear" => {
                        print!("\x1B[2J\x1B[1;1H"); // Clear screen
                        continue;
                    }
                    "help" => {
                        print_help();
                        continue;
                    }
                    "status" => {
                        println!("Tasks executed: {}", task_count - 1);
                        println!("Capabilities active: {}", security::get_active_caps());
                        continue;
                    }
                    _ => {}
                }

                // Route command through security and execution
                router::route(cmd);
            }
            Err(e) => {
                eprintln!("Error reading input: {}", e);
                break;
            }
        }
    }
}

fn print_help() {
    println!("\nOASM Shell Commands:");
    println!("  help      - Show this help");
    println!("  history   - Show command history");
    println!("  status    - Show task count and capabilities");
    println!("  clear     - Clear screen");
    println!("  exit/quit - Exit shell");
    println!("\nExecutive Function Features:");
    println!("  - Numbered prompts track task progression");
    println!("  - History command provides working memory recall");
    println!("  - Structured output reduces cognitive load");
    println!("  - Clear error messages with recovery suggestions");
    println!();
}
