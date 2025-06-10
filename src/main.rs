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

/// Parse the input string into a command and its arguments
/// # Arguments:
/// * `input` - A string slice containing the input command
/// # Returns:
/// * A tuple containing the command as a String and a vector of arguments as Vec<String>
fn parse_input(input: &str) -> (String, Vec<String>) {
    let mut args = Vec::new();
    let mut current = String::new();
    let mut in_single_quote = false;
    let mut in_double_quote = false;
    let mut backslash_happened = false;
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '\'' if !in_double_quote & !backslash_happened=> {
                in_single_quote = !in_single_quote;
            }
            '"' if !backslash_happened & !in_single_quote=> {
                in_double_quote = !in_double_quote;
            }
            ' ' if !in_single_quote & !in_double_quote & !backslash_happened => {
                if !current.is_empty() {
                    args.push(current.clone());
                    current.clear();
                }
            }
            _ if backslash_happened => {
                current.push(c);
                backslash_happened = false;
            }
            '\\' if !in_double_quote & !in_single_quote=> {
                backslash_happened = true;
            }
            _ => {
                current.push(c);
            }
        }
    }

    if !current.is_empty() {
        args.push(current);
    }

    // trim all arguments
    args.iter_mut().for_each(|arg| *arg = arg.trim().to_string());

    let command = args.get(0).cloned().unwrap_or_default();
    let remaining_args = if args.len() > 1 { args[1..].to_vec() } else { vec![] };
    (command, remaining_args)
}

fn main() {

    // Wait for user input
    let mut input = String::new();

    let valid_commands_builtin = vec!["exit", "echo", "type", "pwd", "cd"];
    let valid_commands_executables = get_path_executables();

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        // parse input into two sections
        let (command, args) = parse_input(&input);
        let argument = args.join(" ").trim().to_string(); // used for echo/type/cd etc.


        match  command.as_str() {
            "exit" => {
                exit(0)
            },
            "echo" => {
                println!("{}", argument.trim());
            },
            "type" => {
                if valid_commands_builtin.contains(&argument.as_str()) {
                    println!("{} is a shell builtin", argument);
                } else if valid_commands_executables.contains_key(&argument) {
                    println!("{} is {}", argument, valid_commands_executables.get(&argument).unwrap());
                } else {
                    println!("{}: not found", argument);
                }
            },
            "pwd" => {
                let current_dir = env::current_dir().unwrap();
                println!("{}", current_dir.display());
            },
            "cd" => {
                if !argument.is_empty() {
                    if Path::new(&argument).exists() {
                        env::set_current_dir(&argument).unwrap();
                    } else if argument.eq("~") {
                        let home_dir = env::var("HOME").unwrap_or("".to_string());
                        env::set_current_dir(home_dir).unwrap();
                    }
                    else {

                        println!("cd: {}: No such file or directory", argument);
                    }
                }
            },
            _ if valid_commands_executables.contains_key(&command) => {
                let output = Command::new(command)
                    .args(args) // Pass the rest of the arguments
                    .output().unwrap();
                let stdout = String::from_utf8_lossy(&output.stdout);
                // let stderr = String::from_utf8_lossy(&output.stderr);
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
