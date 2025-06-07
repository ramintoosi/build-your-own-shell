#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::exit;

enum Command {
    Exit(i32),
    Echo(String),
    Unknown(String),
}

fn main() {

    // Wait for user input
    let mut input = String::new();

    let valid_commands = vec!["exit", "echo", "type"];

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        // parse input into two sections
        let parts: Vec<&str> = input.trim().splitn(2, " ").collect();
        let command = parts.get(0).unwrap_or(&"").to_string();
        let argument = parts.get(1).unwrap_or(&"").to_string();

        match  command.as_str() {
            t if t.starts_with("exit") => {
                exit(0)
            },
            t if t.starts_with("echo") => {
                println!("{}", argument);
            },
            t if t.starts_with("type") => {
                if valid_commands.contains(&argument.as_str()) {
                    println!("{} is a shell builtin", argument);
                } else {
                    println!("{}: not found", argument);
                }
            },
            _ => {
                println!("{}: command not found", input);
            }
        }
        input.clear(); // Clear the input buffer for the next command
    }

}
