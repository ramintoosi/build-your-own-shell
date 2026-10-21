use std::collections::HashMap;
use std::env;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

/// Get the list of executables in the PATH environment variable
///
/// # Returns:
/// Vec<String> - A vector of strings containing the names of executables
pub(crate) fn get_path_executables() -> HashMap<String, String> {
    let path = env::var("PATH").unwrap_or("".to_string());
    let mut executables: HashMap<String, String> = HashMap::new();
    for dir in path.split(":") {
        let dir_path = Path::new(dir);
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Some(name) = entry.file_name().to_str() {
                        if executables.contains_key(&name.to_string()) {
                            continue;
                        }
                        // Check if the file is executable
                        if entry.metadata().map_or(false, |m| {
                            m.is_file() && m.permissions().mode() & 0o111 != 0
                        }) {
                            let full_path = dir_path.join(name);
                            executables.insert(name.to_string(), full_path.display().to_string());
                        }
                    }
                }
            }
        }
    }

    executables
}

/// Get the list of files and folders in the current directory
pub(crate) fn get_directory_files_and_folders(requested_path: Option<&str>) -> Vec<String> {
    let path = requested_path.unwrap_or(".");
    let path = Path::new(path);
    if !path.exists() {
        return Vec::new();
    }
    let files = std::fs::read_dir(path).unwrap();
    files
        .map(|file| {
            let entry = file.unwrap();
            if entry.path().is_dir() {
                if requested_path.is_some() {
                    return format!("{}/", entry.path().display().to_string());
                } else {
                    return format!("{}/", entry.file_name().to_str().unwrap().to_string());
                }
            } else {
                if requested_path.is_some() {
                    return entry.path().display().to_string();
                } else {
                    return entry.file_name().to_str().unwrap().to_string();
                }
            }
        })
        .collect()
}
