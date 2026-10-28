mod background;
mod builtins;
mod completer;
mod executor;
mod external_completers;
mod history;
mod parser;
mod path;
mod redirect;

use background::{add_to_background, get_length_of_background_processes, notify_finished_jobs};
use completer::ShellCompleter;
use executor::command_executor;
use history::{add_to_history, load_history_on_startup};

use rustyline::config::Config;
use rustyline::CompletionType;
use rustyline::Editor;

pub fn run() {
    let config = Config::builder()
        .completion_type(CompletionType::List)
        .build();

    let mut rl =
        Editor::<ShellCompleter, rustyline::history::DefaultHistory>::with_config(config).unwrap();
    rl.set_helper(Some(ShellCompleter));

    load_history_on_startup();

    // Wait for user input
    let mut input: String;

    loop {
        let finished_jobs = notify_finished_jobs();
        if !finished_jobs.is_empty() {
            print!("{}", finished_jobs);
        }
        input = rl.readline("$ ").unwrap();

        add_to_history(&input);
        rl.add_history_entry(&input).unwrap();

        // handle pipelines
        let pipelines = input.split(" | ").collect::<Vec<&str>>();
        let mut children = Vec::new();
        let mut previous_stdout = None;

        for (index, command) in pipelines.iter().enumerate() {
            let command_original = Some(command.to_string());

            // check if the command is a background process
            let background = command.trim_end().ends_with('&');
            let command = if background {
                command.trim_end().trim_end_matches('&').trim()
            } else {
                command
            };

            let is_last = index == pipelines.len() - 1;
            previous_stdout = command_executor(
                &command,
                is_last,
                previous_stdout,
                &mut children,
                background,
            );
            if background {
                let child = children.pop().unwrap();
                let child_id = child.id();
                add_to_background(child, command_original.unwrap());
                println!("[{}] {}", get_length_of_background_processes(), child_id);
            }
        }
        for mut child in children {
            child.wait().unwrap();
        }

        input.clear(); // Clear the input buffer for the next command
    }
}
