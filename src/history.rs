use std::sync::{Mutex};

fn store() -> &'static Mutex<Vec<String>> {
    static HISTORY: Mutex<Vec<String>> = Mutex::new(Vec::new());
    &HISTORY
}

pub(crate) fn add_to_history(command: &str) {
    let mut history = store().lock().unwrap();
    history.push(command.to_string());
}


pub(crate) fn format_history(limit: Option<usize>) -> String {
    let history = store().lock().unwrap();
    if let Some(limit) = limit {
        history.iter().rev().take(limit).rev().enumerate().map(|(index, cmd)| format!("    {}  {}\n", index + history.len() - limit + 1, cmd)).collect()
    }
    else {
        history.iter().enumerate().map(|(index, cmd)| format!("    {}  {}\n", index + 1, cmd)).collect()
    }
}