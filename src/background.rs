/*
This module is responsible for managing background processes.
*/

use std::process::Child;
use std::sync::Mutex;

use crate::background;

struct Process {
    command: String,
    child: Child,
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
    });
}

pub(crate) fn print_background_processes() -> String {
    let mut background = store().lock().unwrap();
    let length = background.processes.len();
    let mut out_text = String::new();
    for (index, process) in background.processes.iter_mut().enumerate() {
        let is_running = match process.child.try_wait() {
            Ok(Some(_)) => "Done",
            Ok(None) => "Running",
            Err(_) => "Error",
        };
        // a 0, 1, 2 variable that shows if its last, second last, or not
        let sign = if index == length - 1 {
            "+"
        } else if index == length - 2 {
            "-"
        } else {
            ""
        };
        out_text.push_str(&format!(
            "[{}]{}  {}\t{}\n",
            index + 1,
            sign,
            is_running,
            process.command
        ));
    }
    out_text
}

pub(crate) fn get_length_of_background_processes() -> usize {
    let background = store().lock().unwrap();
    background.processes.len()
}
