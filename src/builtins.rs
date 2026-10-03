use std::collections::HashMap;
use std::env;
use std::process::{exit, Stdio};

use crate::history::{format_history, load_history, save_history, save_history_on_exit};
use crate::redirect::{handle_output, output_pipe};

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
) -> Option<Stdio> {
    match command {
        "exit" => {
            save_history_on_exit();
            exit(0)
        }
        "echo" => {
            let text = format!("{}\n", argument);
            emit(&text, print_output, stdout_path, stdout_append)
        }
        "type" => {
            let text = if is_builtin(argument) {
                format!("{} is a shell builtin\n", argument)
            } else if let Some(path) = executables.get(argument) {
                format!("{} is {}\n", argument, path)
            } else {
                format!("{}: not found\n", argument)
            };
            emit(&text, print_output, stdout_path, stdout_append)
        }
        "pwd" => {
            let text = format!("{}\n", env::current_dir().unwrap().to_string_lossy());
            emit(&text, print_output, stdout_path, stdout_append)
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
                    return emit(&output, print_output, stdout_path, stdout_append);
                }
            }
            None
        }
        "history" => {
            let history: String;
            if argument.is_empty() {
                history = format_history(None);
                emit(&history, print_output, stdout_path, stdout_append)
            } else if argument.starts_with("-r") {
                let file_path = argument.split_at(3).1;
                load_history(file_path);
                None
            } else if argument.starts_with("-w") {
                let file_path = argument.split_at(3).1;
                save_history(file_path, false);
                None
            } else if argument.starts_with("-a") {
                let file_path = argument.split_at(3).1;
                save_history(file_path, true);
                None
            } else {
                let index = argument.parse::<usize>().unwrap();
                history = format_history(Some(index));
                emit(&history, print_output, stdout_path, stdout_append)
            }
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
