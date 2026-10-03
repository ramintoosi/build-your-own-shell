use std::{io::Write, sync::Mutex};

struct History {
    history: Vec<String>,
    index: usize,
    default_file_path: String,
}

impl History {
    const fn new() -> Self {
        Self {
            history: Vec::new(),
            index: 0,
            default_file_path: String::new(),
        }
    }
}

fn store() -> &'static Mutex<History> {
    static HISTORY: Mutex<History> = Mutex::new(History::new());
    &HISTORY
}

pub(crate) fn load_history_on_startup() {
    let default_file_path =
        std::env::var("HISTFILE").unwrap_or_else(|_| "/tmp/.my_shell_history".to_string());
    if !std::path::Path::new(&default_file_path).exists() {
        std::fs::File::create(&default_file_path).unwrap();
    }
    load_history(&default_file_path);
    let mut history = store().lock().unwrap();
    history.default_file_path = default_file_path;
    history.index = history.history.len();
    drop(history);
}

pub(crate) fn save_history_on_exit() {
    let history = store().lock().unwrap();
    let default_file_path = history.default_file_path.clone();
    drop(history);
    save_history(&default_file_path, true);
}

pub(crate) fn add_to_history(command: &str) {
    let mut history = store().lock().unwrap();
    history.history.push(command.to_string());
}

pub(crate) fn format_history(limit: Option<usize>) -> String {
    let history = store().lock().unwrap();
    if let Some(limit) = limit {
        let start = history.history.len() - limit;
        history
            .history
            .iter()
            .skip(start)
            .enumerate()
            .map(|(index, cmd)| format!("    {}  {}\n", start + index, cmd))
            .collect()
    } else {
        history
            .history
            .iter()
            .enumerate()
            .map(|(index, cmd)| format!("    {}  {}\n", index + 1, cmd))
            .collect()
    }
}

pub(crate) fn load_history(file_path: &str) {
    for line in std::fs::read_to_string(file_path).unwrap().lines() {
        add_to_history(line);
    }
}

pub(crate) fn save_history(file_path: &str, append: bool) {
    let mut history = store().lock().unwrap();
    let mut file = if append {
        std::fs::OpenOptions::new()
            .append(true)
            .open(file_path)
            .unwrap()
    } else {
        std::fs::File::create(file_path).unwrap()
    };
    let skip = if append { history.index } else { 0 };
    for line in history.history.iter().skip(skip) {
        file.write_all(line.as_bytes()).unwrap();
        file.write_all(b"\n").unwrap();
    }
    if append {
        history.index = history.history.len();
    }
}
