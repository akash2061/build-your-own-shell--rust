#[allow(unused_imports)]
use std::io::{self, Write};

fn is_builtin(command: &str) -> bool {
    match command {
        "echo" | "exit" | "type" => true,
        _ => false,
    }
}

fn is_executable(command: &str) -> Option<String> {
    // Check if the command is available in the system's PATH
    if let Some(paths) = std::env::var_os("PATH") {
        for path in std::env::split_paths(&paths) {
            let full_path = path.join(command);
            if full_path.is_file() {
                // Return the full path to the executable
                return Some(full_path.to_string_lossy().into_owned());
            }
        }
    }
    None
}
fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        // Split input into command and arguments
        let input = input.trim();
        let mut parts = input.split_whitespace();
        let command = parts.next();
        let args: Vec<&str> = parts.collect();

        match command {
            Some("exit") => break,
            Some("echo") => {
                println!("{}", args.join(" "));
            }
            Some("type") => {
                if args.is_empty() {
                    println!("type: argument required");
                } else {
                    for arg in args {
                        if is_builtin(arg) {
                            println!("{} is a shell builtin", arg);
                        } else if let Some(executable_path) = is_executable(arg) {
                            println!("{} is {}", arg, executable_path);
                        } else {
                            println!("{}: command not found", arg);
                        }
                    }
                }
            }
            Some(cmd) => println!("{}: command not found", cmd),
            None => continue,
        };
    }
}
