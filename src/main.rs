#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::exit;
use std::env;
use std::collections::HashMap;
use std::path::Path;
use std::process::Command;

/// Get the list of executables in the PATH environment variable
///
/// # Returns:
/// Vec<String> - A vector of strings containing the names of executables
fn get_path_executables() ->  HashMap<String, String>{

    let path = env::var("PATH").unwrap_or("".to_string());
    let mut executables: HashMap<String, String> = HashMap::new();
    for dir in path.split(":") {
        let dir_path = Path::new(dir);
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Some(name) = entry.file_name().to_str() {
                        if executables.contains_key(&name.to_string()) {
                            continue;
                        }
                        // Check if the file is executable
                        if entry.metadata().map_or(false, |m| m.is_file()) {
                            let full_path = dir_path.join(name);
                            executables.insert(name.to_string(), full_path.display().to_string());
                        }
                    }
                }
            }
        }
    }

    executables

}

fn main() {

    // Wait for user input
    let mut input = String::new();

    let valid_commands_builtin = vec!["exit", "echo", "type"];
    let valid_commands_executables = get_path_executables();

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
                if valid_commands_builtin.contains(&argument.as_str()) {
                    println!("{} is a shell builtin", argument);
                } else if valid_commands_executables.contains_key(&argument) {
                    println!("{} is {}", argument, valid_commands_executables.get(&argument).unwrap());
                }
                else {
                    println!("{}: not found", argument);
                }
            },
            _ if valid_commands_executables.contains_key(&command) => {
                let full_path = valid_commands_executables.get(&command).unwrap();
                let args = argument.split(" ");
                // println!("{:?} - {:?}", full_path, args);
                let output = Command::new(command)
                    .args(args) // Pass the rest of the arguments
                    .output().unwrap();
                let stdout = String::from_utf8_lossy(&output.stdout);
                let stderr = String::from_utf8_lossy(&output.stderr);
                if !stdout.is_empty() {
                    println!("{}", &stdout.trim());
                }
                // if !stderr.is_empty() {
                //     println!("{:?}", &stderr.trim());
                // }

            },
            _ => {
                println!("{}: command not found", input.trim());
            }
        }
        input.clear(); // Clear the input buffer for the next command
    }

}
