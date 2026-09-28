use std::collections::HashMap;
use std::env;
use std::process::{exit, Stdio};

use crate::redirect::{handle_output, output_pipe};
use crate::history::format_history;

pub(crate) const BUILTINS: [&str; 6] = ["exit", "echo", "type", "pwd", "cd", "history"];

pub(crate) fn is_builtin(command: &str) -> bool {
    BUILTINS.contains(&command)
}

pub(crate) fn run_builtin(
    command: &str,
    argument: &str,
    print_output: bool,
    stdout_path: &Option<String>,
    stdout_append: bool,
    executables: &HashMap<String, String>,
) -> Option<Option<Stdio>> {
    match command {
        "exit" => exit(0),
        "echo" => {
            let text = format!("{}\n", argument);
            Some(emit(&text, print_output, stdout_path, stdout_append))
        }
        "type" => {
            let text = if is_builtin(argument) {
                format!("{} is a shell builtin\n", argument)
            } else if let Some(path) = executables.get(argument) {
                format!("{} is {}\n", argument, path)
            } else {
                format!("{}: not found\n", argument)
            };
            Some(emit(&text, print_output, stdout_path, stdout_append))
        }
        "pwd" => {
            let text = format!("{}\n", env::current_dir().unwrap().to_string_lossy());
            Some(emit(&text, print_output, stdout_path, stdout_append))
        }
        "cd" => {
            if !argument.is_empty() {
                if std::path::Path::new(argument).exists() {
                    env::set_current_dir(argument).unwrap();
                } else if argument == "~" {
                    let home = env::var("HOME").unwrap_or_default();
                    env::set_current_dir(home).unwrap();
                } else {
                    let output = format!("cd: {}: No such file or directory\n", argument);
                    return Some(emit(&output, print_output, stdout_path, stdout_append));
                }
            }
            Some(None)
        }
        "history" => {
            let history: String;
            if argument.is_empty() {
                history = format_history(None);
            }
            else {
                let index = argument.parse::<usize>().unwrap();
                history = format_history(Some(index));
            }
            
            Some(emit(&history, print_output, stdout_path, stdout_append))
        }
        _ => None, // not a builtin
    }
}
fn emit(
    text: &str,
    print_output: bool,
    stdout_path: &Option<String>,
    stdout_append: bool,
) -> Option<Stdio> {
    if print_output {
        handle_output(text, stdout_path, stdout_append);
        None
    } else {
        Some(output_pipe(text))
    }
}
