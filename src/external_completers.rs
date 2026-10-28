use std::collections::HashMap;
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
