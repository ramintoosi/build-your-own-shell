#[allow(unused_imports)]
use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;
use std::process::{exit};
use std::{env};
use std::collections::HashMap;
use std::path::Path;
use std::process::Command;
use std::fs::File;
use rustyline::completion::{Completer, Pair};
use rustyline::{Context};
use rustyline::Result;
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::validate::Validator;
use rustyline::Helper;
use rustyline::Editor;

/// autocomplete the command
struct ShellCompleter;

impl Completer for ShellCompleter {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &Context<'_>,
    ) -> Result<(usize, Vec<Pair>)> {
        let line_to_cursor = &line[..pos];

        let start = line_to_cursor.rfind(' ').map_or(0, |i| i + 1);
        let word = &line_to_cursor[start..];

        let builtins = vec!["exit", "echo", "type", "pwd", "cd"];
        let executables = get_path_executables();
        let all_commands = [builtins, executables.keys().map(|k| k.as_str()).collect::<Vec<&str>>()].concat();
        let mut matches = Vec::new();

        for &cmd in &all_commands {
            if cmd.starts_with(word) {
                matches.push(Pair {
                    display: cmd.to_string(),
                    replacement: format!("{} ", cmd)
                })
            }
        }
        Ok((start, matches))
    }
}

// Empty trait implementations to satisfy the Helper requirements
impl Highlighter for ShellCompleter {}
impl Hinter for ShellCompleter {
    type Hint = String;

    fn hint(&self, _: &str, _: usize, _: &Context<'_>) -> Option<String> {
        None
    }
}
impl Validator for ShellCompleter {}
impl Helper for ShellCompleter {}


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
fn parse_input(input: &str) -> (String, Vec<String>, String, Option<String>, bool, Option<String>, bool) {
    let mut args = Vec::new();
    let mut current = String::new();
    let mut in_single_quote = false;
    let mut in_double_quote = false;
    let mut backslash_happened = false;

    let input = input.replace("1>", ">");
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
    let mut redirect_stdout_mode: bool = false;
    let mut redirect_stderr_mode: bool = false;

    if argument.contains(">") {
        let argument_clone = argument.clone();

        let split_pattern = if argument.contains(" > ") | argument.contains(" >> ") {
            if argument.contains(" >> ") { 
                redirect_stdout_mode = true; 
                " >> "
            } else {
                " > "
            }
        } else {
            if argument.contains(" 2>> ") { 
                redirect_stderr_mode = true; 
                " 2>> "
            } else {
                " 2> "
            }
        };

        let argument_split = argument_clone.split(split_pattern).collect::<Vec<&str>>();
        argument = argument_split.get(0).cloned().unwrap_or(&"").trim().to_string();
        // remove the redirect part from remaining_args
        if let Some(index) = remaining_args.iter().position(|s| (s == ">") | (s == "2>") | (s == ">>") | (s == "2>>")) {
            // Truncate the vector, keeping elements *before* the index
            remaining_args.truncate(index);
            // println!("argument_split: {:?}", argument_split);
            if (split_pattern == " > ") | (split_pattern == " >> ") {
                redirect_stdout = Some(argument_split.get(1).unwrap_or(&"").to_string());
            } else {
                redirect_stderr = Some(argument_split.get(1).unwrap_or(&"").to_string());
            }
        }
    };
    
    (command, remaining_args, argument, redirect_stdout, redirect_stdout_mode, redirect_stderr, redirect_stderr_mode)
}

fn handle_output(output: &str, redirect: &Option<String>, redirect_mode: bool) {
    if let Some(file_path) = redirect {
        let mut hfile: File;
        if redirect_mode {
            hfile = std::fs::OpenOptions::new().create(true).append(true).open(file_path).unwrap();
        } else {
            hfile = std::fs::OpenOptions::new().create(true).write(true).open(file_path).unwrap();
        }
        hfile.write_all(output.as_bytes()).unwrap();
        hfile.flush().unwrap();
    } else {
        println!("{}", output.trim());
    }
}

fn main() {

    let mut rl = Editor::<ShellCompleter, rustyline::history::DefaultHistory>::new().unwrap();
    rl.set_helper(Some(ShellCompleter));

    // Wait for user input
    let mut input: String;

    let valid_commands_builtin = vec!["exit", "echo", "type", "pwd", "cd"];
    let valid_commands_executables = get_path_executables();

    loop {
        // print!("$ ");
        // io::stdout().flush().unwrap();
        // io::stdin().read_line(&mut input).unwrap();
        input = rl.readline("$ ").unwrap();
        // parse input into two sections
        let (command, args, argument, redirect_stdout, redirect_stdout_mode, redirect_stderr, redirect_stderr_mode) = parse_input(&input);

        // we create the files if not exists for redirect no matter what
        if let Some(file_path) = &redirect_stdout {
            // println!("redirect_stdout: {}", file_path);
            if !std::path::Path::new(file_path).exists() {
                std::fs::File::create(file_path).unwrap();
            }
        }
        if let Some(file_path) = &redirect_stderr {
            // println!("redirect_stderr: {}", file_path);
            if !std::path::Path::new(file_path).exists() {
                std::fs::File::create(file_path).unwrap();
            }
        }
        
        match  command.as_str() {
            "exit" => {
                exit(0)
            },
            "echo" => {
                handle_output(&format!("{}\n", argument), &redirect_stdout, redirect_stdout_mode);
            },
            "type" => {
                if valid_commands_builtin.contains(&argument.as_str()) {
                    let output = format!("{} is a shell builtin\n", argument);
                    handle_output(&output, &redirect_stdout, redirect_stdout_mode);
                } else if valid_commands_executables.contains_key(&argument) {
                    let output = format!("{} is {}\n", argument, valid_commands_executables.get(&argument).unwrap());
                    handle_output(&output, &redirect_stdout, redirect_stdout_mode);
                } else {
                    let output = format!("{}: not found\n", argument);
                    handle_output(&output, &redirect_stdout, redirect_stdout_mode);
                }
            },
            "pwd" => {
                let current_dir = env::current_dir().unwrap();
                handle_output(&format!("{}\n", current_dir.to_string_lossy()), &redirect_stdout, redirect_stdout_mode);

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
                        let output = format!("cd: {}: No such file or directory\n", argument);
                        handle_output(&output, &redirect_stdout, redirect_stdout_mode);
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
                    handle_output(&stdout, &redirect_stdout, redirect_stdout_mode);
                }
                if !stderr.is_empty() {
                    handle_output(&stderr, &redirect_stderr, redirect_stderr_mode);
                }

            },
            _ => {
                let output = format!("{}: command not found", input.trim());
                handle_output(&output, &redirect_stdout, redirect_stdout_mode);
            }
        }
        input.clear(); // Clear the input buffer for the next command
    }

}
