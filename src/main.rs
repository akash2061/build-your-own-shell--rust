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
            Some(cmd) => println!("{}: command not found", cmd),
            None => continue,
        };
    }
}
