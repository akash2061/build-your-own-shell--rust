#[allow(unused_imports)]
use std::io::{self, Write};

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
                        match arg {
                            "echo" | "exit" | "type" => {
                                println!("{} is a shell builtin", arg);
                            }
                            _ => {
                                println!("{} not found", arg);
                            }
                        }
                    }
                }
            }
            Some(cmd) => println!("{}: command not found", cmd),
            None => continue,
        };
    }
}
