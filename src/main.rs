#[allow(unused_imports)]
use std::io::{self, Write};

fn is_builtin(command: &str) -> bool {
    match command {
        "echo" | "exit" | "type" => true,
        _ => false,
    }
}

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

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
                    let path_env = std::env::var("PATH").unwrap_or_default();
                    for arg in args {
                        if is_builtin(arg) {
                            println!("{} is a shell builtin", arg);
                        } else {
                            let split = &mut path_env.split(':');
                            if let Some(path) =
                                split.find(|path| std::fs::metadata(format!("{}/{}", path, arg)).is_ok())
                            {
                                println!("{} is {}/{}", arg, path, arg);
                            } else {
                                println!("{}: not found", arg);
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
