mod builtins;
mod completer;
mod executor;
mod parser;
mod path;
mod redirect;
mod history;

use completer::ShellCompleter;
use executor::command_executor;
use history::add_to_history;

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

    // Wait for user input
    let mut input: String;

    loop {
        input = rl.readline("$ ").unwrap();
        
        add_to_history(&input);
        
        // handle pipelines
        let pipelines = input.split(" | ").collect::<Vec<&str>>();
        let mut children = Vec::new();
        let mut previous_stdout = None;

        for (index, command) in pipelines.iter().enumerate() {
            let is_last = index == pipelines.len() - 1;
            previous_stdout = command_executor(&command, is_last, previous_stdout, &mut children);
        }

        for mut child in children {
            child.wait().unwrap();
        }

        input.clear(); // Clear the input buffer for the next command
    }
}
