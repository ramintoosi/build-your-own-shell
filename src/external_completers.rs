use std::collections::HashMap;
use std::process::Command;
use std::sync::{LazyLock, Mutex};

pub(crate) fn get_external_completers() -> &'static Mutex<HashMap<String, String>> {
    static EXTERNAL_COMPLETERS: LazyLock<Mutex<HashMap<String, String>>> =
        LazyLock::new(|| Mutex::new(HashMap::new()));
    &EXTERNAL_COMPLETERS
}

pub(crate) fn check_if_external_completer_exists(command: &str) -> Option<String> {
    if let Some(path) = get_external_completers().lock().unwrap().get(command) {
        return Some(path.to_string());
    }
    None
}

pub(crate) fn add_external_completer(command: &str, path: &str) {
    get_external_completers()
        .lock()
        .unwrap()
        .insert(command.to_string(), path.to_string());
}

pub(crate) fn run_external_completer(command: &str) -> Vec<String> {
    if let Some(path) = check_if_external_completer_exists(command) {
        let output = Command::new(path).arg(command).output().unwrap();
        let output_string = String::from_utf8(output.stdout).unwrap();
        return output_string
            .split('\n')
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect();
    }
    Vec::new()
}
