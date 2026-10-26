/*
This module is responsible for managing background processes.
*/

use std::process::Child;
use std::sync::Mutex;

struct Process {
    command: String,
    child: Child,
    is_done: bool,
}

struct Background {
    processes: Vec<Process>,
}

impl Background {
    const fn new() -> Self {
        Self {
            processes: Vec::new(),
        }
    }
}

fn store() -> &'static Mutex<Background> {
    static BACKGROUND: Mutex<Background> = Mutex::new(Background::new());
    &BACKGROUND
}

pub(crate) fn add_to_background(child: Child, command: String) {
    let mut background = store().lock().unwrap();
    background.processes.push(Process {
        command: command,
        child: child,
        is_done: false,
    });
}

pub(crate) fn print_background_processes() -> String {
    let mut background = store().lock().unwrap();
    
    let mut out_text = String::new();
    // let mut processes_to_remove = Vec::new();

    let mut output_vec: Vec<Vec<String>> = Vec::new();

    for (index, process) in background.processes.iter_mut().enumerate() {
        
        if process.is_done {
            continue;
        }

        let is_running = match process.child.try_wait() {
            Ok(Some(_)) => "Done",
            Ok(None) => "Running",
            Err(_) => "Error",
        };

        let command_to_print = if is_running == "Done" {
            process.is_done = true;
            process.command.trim_end().trim_end_matches('&').trim_end()
        } else {
            &process.command
        };

        output_vec.push(vec![
            (index + 1).to_string(),
            is_running.to_string(),
            command_to_print.to_string(),
        ]);

    }

    let length = output_vec.len();

    for (index, row) in output_vec.iter().enumerate() {
        let sign = if index == length - 1 {
            "+"
        } else if index == length - 2 {
            "-"
        } else {
            ""
        };

        out_text.push_str(&format!("[{}]{}  {}\t{}\n", row[0], sign, row[1], row[2]));
    }
    
    out_text
}

pub(crate) fn get_length_of_background_processes() -> usize {
    let background = store().lock().unwrap();
    background.processes.len()
}
