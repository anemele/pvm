use crate::utils;
use std::fs;
use std::path::Path;

pub fn remove(venv_path: &Path, name: &String) -> i32 {
    let path = venv_path.join(name);
    if !path.exists() {
        eprintln!("No env `{name}` exists.");
        return 1;
    }

    if path.is_file() {
        eprintln!("File with the same name exists.");
        return 1;
    }

    if !utils::is_valid_env(path.as_path()) {
        eprintln!("Invalid env `{name}`");
        return 1;
    }

    match fs::remove_dir_all(path) {
        Ok(_) => {
            println!("Removed env `{name}`");
            0
        }
        Err(_) => {
            println!("Failed to remove `{name}`");
            1
        }
    }
}
