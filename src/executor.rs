use std::process::Command;
use std::process::Stdio;

use crate::builtins::{is_builtin, run_builtin};
use crate::parser::parse_input;
use crate::path::get_path_executables;
use crate::redirect::stream_pipe;
use crate::redirect::{handle_output, output_pipe};

fn spawn_command(
    program: &str,
    args: &[String],
    previous_stdio: Option<Stdio>,
) -> std::process::Child {
    let mut cmd = Command::new(program);
    cmd.args(args);

    if let Some(stdout) = previous_stdio {
        cmd.stdin(stdout);
    }

    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());
    cmd.spawn().unwrap()
}

pub(crate) fn command_executor(
    command: &str,
    print_output: bool,
    previous_stdio: Option<Stdio>,
    children: &mut Vec<std::process::Child>,
) -> Option<Stdio> {
    let valid_commands_executables = get_path_executables();

    // parse input into two sections
    let parsed_command = parse_input(command);

    let mut stdout_path: Option<String> = None;
    let mut stderr_path: Option<String> = None;
    let mut stdout_append: bool = false;
    let mut stderr_append: bool = false;

    // we create the files if not exists for redirect no matter what
    if let Some(redirect) = &parsed_command.stdout {
        let file_path = &redirect.path;
        if !std::path::Path::new(file_path).exists() {
            std::fs::File::create(file_path).unwrap();
        }
        stdout_path = Some(file_path.clone());
        stdout_append = redirect.append;
    }
    if let Some(redirect) = &parsed_command.stderr {
        let file_path = &redirect.path;
        // println!("redirect_stderr: {}", file_path);
        if !std::path::Path::new(file_path).exists() {
            std::fs::File::create(file_path).unwrap();
        }
        stderr_path = Some(file_path.clone());
        stderr_append = redirect.append;
    }

    match parsed_command.command.as_str() {
        _ if is_builtin(&parsed_command.command) => {
            let result = run_builtin(
                &parsed_command.command,
                &parsed_command.argument,
                print_output,
                &stdout_path,
                stdout_append,
                &valid_commands_executables,
            );
            result.unwrap()
        }
        _ if valid_commands_executables.contains_key(&parsed_command.command) => {
            let mut child = spawn_command(
                &parsed_command.command,
                &parsed_command.args,
                previous_stdio,
            );

            if print_output {
                let stdout_thread = child
                    .stdout
                    .take()
                    .map(|pipe| stream_pipe(pipe, stdout_path, stdout_append));
                let output = child.wait_with_output().unwrap();
                if let Some(thread) = stdout_thread {
                    thread.join().unwrap();
                }
                let stderr = String::from_utf8_lossy(&output.stderr);
                if !stderr.is_empty() {
                    handle_output(&stderr, &stderr_path, stderr_append);
                }

                if print_output {
                    None
                } else {
                    Some(output_pipe(""))
                }
            } else {
                let next_stdout = child.stdout.take().map(Stdio::from);
                children.push(child);
                next_stdout
            }
        }
        _ => {
            let output = format!("{}: command not found\n", command.trim());
            if print_output {
                handle_output(&output, &stdout_path, stdout_append);
            }
            None
        }
    }
}
