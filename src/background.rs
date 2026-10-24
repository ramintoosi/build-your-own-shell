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
    let length = background.processes.len();
    let mut out_text = String::new();
    // let mut processes_to_remove = Vec::new();

    for (index, process) in background.processes.iter_mut().enumerate() {
        
        if process.is_done {
            continue;
        }

        let is_running = match process.child.try_wait() {
            Ok(Some(_)) => "Done",
            Ok(None) => "Running",
            Err(_) => "Error",
        };
        // A 0, 1, 2 variable that shows if its last, second last, or not
        let sign = if index == length - 1 {
            "+"
        } else if index == length - 2 {
            "-"
        } else {
            ""
        };
        let command_to_print = if is_running == "Done" {
            process.is_done = true;
            process.command.trim_end().trim_end_matches('&').trim_end()
        } else {
            &process.command
        };
        out_text.push_str(&format!(
            "[{}]{}  {}\t{}\n",
            index + 1,
            sign,
            is_running,
            command_to_print,
        ));
    }
    
    out_text
}

pub(crate) fn get_length_of_background_processes() -> usize {
    let background = store().lock().unwrap();
    background.processes.len()
}
