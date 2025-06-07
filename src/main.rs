#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::exit;

enum Command {
    Exit(i32),
    Echo(String),
    Unknown(String),
}

fn print_not_found(command: &str) {
    println!("{}: command not found", command);
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
                    print_not_found(argument.as_str());
                }
            },
            _ => {
                print_not_found(input.as_str());
            }
        }
        input.clear(); // Clear the input buffer for the next command
    }

}
