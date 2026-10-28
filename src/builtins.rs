use std::collections::HashMap;
use std::env;
use std::process::{exit, Stdio};

use crate::background::print_background_processes;
use crate::external_completers::{
    add_external_completer, check_if_external_completer_exists, unregister_external_completer,
};
use crate::history::{format_history, load_history, save_history, save_history_on_exit};
use crate::redirect::{handle_output, output_pipe};

pub(crate) const BUILTINS: [&str; 9] = [
    "exit", "echo", "type", "pwd", "cd", "history", "jobs", "complete", "declare"
];

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
        "jobs" => {
            let background_processes = print_background_processes();
            emit(
                &background_processes,
                print_output,
                stdout_path,
                stdout_append,
            )
        }
        "complete" => {
            if argument.starts_with("-p") {
                let command = argument.split_at(3).1;
                if let Some(path) = check_if_external_completer_exists(command) {
                    let output = format!("complete -C '{}' {}\n", path, command);
                    emit(&output, print_output, stdout_path, stdout_append)
                } else {
                    let output = format!("complete: {}: no completion specification\n", command);
                    return emit(&output, print_output, stdout_path, stdout_append);
                }
            } else if argument.starts_with("-C") {
                let arg_split = argument.split(" ").collect::<Vec<&str>>();
                let command = arg_split[2];
                let path = arg_split[1];
                add_external_completer(command, path);
                None
            } else if argument.starts_with("-r") {
                let command = argument.split_at(3).1;
                unregister_external_completer(command);
                None
            } else {
                None
            }
        }
        "declare" => {
            if argument.starts_with("-p") {
                let variable_name = argument.split_at(3).1;
                if let Ok(value) = env::var(variable_name) {
                    let output = format!("declare -- {}=\"{}\"\n", variable_name, value);
                    emit(&output, print_output, stdout_path, stdout_append)
                } else {
                    let output = format!("declare: {}: not found\n", variable_name);
                    emit(&output, print_output, stdout_path, stdout_append)
                }
            } else if argument.contains("=") {
                let split_arg = argument.split("=").collect::<Vec<&str>>();
                let variable_name = split_arg[0];
                let value = split_arg[1];
                env::set_var(variable_name, value);
                None
            }
            else {
                None
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
