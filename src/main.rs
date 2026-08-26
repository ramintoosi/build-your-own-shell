#[allow(unused_imports)]
use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;
use std::process::{exit};
use std::{env};
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
                        if entry.metadata().map_or(false, |m| m.is_file() && m.permissions().mode() & 0o111 != 0) {
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
fn parse_input(input: &str) -> (String, Vec<String>, String, Option<String>, Option<String>) {
    let mut args = Vec::new();
    let mut current = String::new();
    let mut in_single_quote = false;
    let mut in_double_quote = false;
    let mut backslash_happened = false;

    let input = input.replace(" 1> ", " > ");
    let mut chars = input.chars().peekable();


    let special_chars = ['\\', '$', '"', '"'];

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
            _ if backslash_happened & !in_double_quote => {
                current.push(c);
                backslash_happened = false;
            }
            '\\' if !in_single_quote & !backslash_happened => {
                backslash_happened = true;
            }
            _ if backslash_happened & in_double_quote => {
                if special_chars.contains(&c) {
                    current.push(c);
                } else {
                    current.push('\\');
                    current.push(c);
                }
                backslash_happened = false;
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
    let mut remaining_args = if args.len() > 1 { args[1..].to_vec() } else { vec![] };
    let mut argument = remaining_args.join(" ").trim().to_string(); // used for echo/type/cd etc.
    // check if redirect is present
    let mut redirect_stdout: Option<String> = None;
    let mut redirect_stderr: Option<String> = None;

    if argument.contains(" > ") | argument.contains(" 2> ") {
        let argument_clone = argument.clone();
        let split_pattern = if argument.contains(" > ") {
            " > "
        } else {
            " 2> "
        };
        let argument_split = argument_clone.split(split_pattern).collect::<Vec<&str>>();
        argument = argument_split.get(0).cloned().unwrap_or(&"").trim().to_string();
        // remove the redirect part from remaining_args
        if let Some(index) = remaining_args.iter().position(|s| (s == ">") | (s == "2>")) {
            // Truncate the vector, keeping elements *before* the index
            remaining_args.truncate(index);
            if split_pattern == " > " {
                redirect_stdout = Some(argument_split.get(1).unwrap_or(&"").to_string());
            } else {
                redirect_stderr = Some(argument_split.get(1).unwrap_or(&"").to_string());
            }
        }
    };
    
    (command, remaining_args, argument, redirect_stdout, redirect_stderr)
}

fn handle_output(output: &str, redirect: &Option<String>) {
    if let Some(file_path) = redirect {
        std::fs::write(file_path, output).unwrap();
    } else {
        println!("{}", output.trim());
    }
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
        let (command, args, argument, redirect_stdout, redirect_stderr) = parse_input(&input);

        // we create the files for redirect no matter what
        if let Some(file_path) = &redirect_stdout {
            std::fs::File::create(file_path).unwrap();
        }
        if let Some(file_path) = &redirect_stderr {
            std::fs::File::create(file_path).unwrap();
        }
        
        match  command.as_str() {
            "exit" => {
                exit(0)
            },
            "echo" => {
                handle_output(&argument, &redirect_stdout);
            },
            "type" => {
                if valid_commands_builtin.contains(&argument.as_str()) {
                    let output = format!("{} is a shell builtin", argument);
                    handle_output(&output, &redirect_stdout);
                } else if valid_commands_executables.contains_key(&argument) {
                    let output = format!("{} is {}", argument, valid_commands_executables.get(&argument).unwrap());
                    handle_output(&output, &redirect_stdout);
                } else {
                    let output = format!("{}: not found", argument);
                    handle_output(&output, &redirect_stdout);
                }
            },
            "pwd" => {
                let current_dir = env::current_dir().unwrap();
                handle_output(&current_dir.to_string_lossy(), &redirect_stdout);

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
                        let output = format!("cd: {}: No such file or directory", argument);
                        handle_output(&output, &redirect_stdout);
                    }
                }
            },
            _ if valid_commands_executables.contains_key(&command) => {
                let output = Command::new(command)
                    .args(args) // Pass the rest of the arguments
                    .output().unwrap();
                let stdout = String::from_utf8_lossy(&output.stdout);
                let stderr = String::from_utf8_lossy(&output.stderr);
                if !stdout.is_empty() {
                    handle_output(&stdout, &redirect_stdout);
                }
                if !stderr.is_empty() {
                    handle_output(&stderr, &redirect_stderr);
                }

            },
            _ => {
                let output = format!("{}: command not found", input.trim());
                handle_output(&output, &redirect_stdout);
            }
        }
        input.clear(); // Clear the input buffer for the next command
    }

}
