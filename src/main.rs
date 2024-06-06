use std::env;
#[allow(unused_imports)]
use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;
use std::process::{Command, Stdio};

fn is_builtin(command: &str) -> bool {
    match command {
        "echo" | "exit" | "type" | "pwd" | "cd" => true,
        _ => false,
    }
}

fn find_executable(command: &str) -> Option<String> {
    if let Some(paths) = std::env::var_os("PATH") {
        for path in std::env::split_paths(&paths) {
            let full_path = path.join(command);
            if full_path.is_file() && is_executable(&full_path) {
                return Some(full_path.to_string_lossy().into_owned());
            }
        }
    }
    None
}

fn is_executable(path: &std::path::Path) -> bool {
    path.metadata()
        .map(|m| m.is_file() && m.permissions().mode() & 0o111 != 0)
        .unwrap_or(false)
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
            Some("pwd") => match env::current_dir() {
                Ok(path) => println!("{}", path.display()),
                Err(e) => eprintln!("pwd: error: {}", e),
            },
            Some("cd") => {
                if args.is_empty() {
                    eprintln!("cd: missing operand");
                } else {
                    let new_dir = args[0];
                    if let Err(e) = env::set_current_dir(new_dir) {
                        eprintln!("{}: No such file or directory", new_dir);
                    }
                }
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
                            if let Some(path) = split
                                .find(|path| std::fs::metadata(format!("{}/{}", path, arg)).is_ok())
                            {
                                println!("{} is {}/{}", arg, path, arg);
                            } else {
                                println!("{}: not found", arg);
                            }
                        }
                    }
                }
            }
            Some(cmd) => {
                if let Some(executable_path) = find_executable(cmd) {
                    let mut child = Command::new(executable_path)
                        .args(&args)
                        .stdout(Stdio::inherit())
                        .stderr(Stdio::inherit())
                        .spawn()
                        .expect("failed to execute command");

                    let _ = child.wait();
                } else {
                    println!("{}: command not found", cmd);
                }
            }
            None => continue,
        };
    }
}
