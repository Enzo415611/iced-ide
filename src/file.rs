use std::{fs, path::PathBuf};

use rfd::FileDialog;

pub fn open_file() -> PathBuf {
    if let Some(path) = FileDialog::new().pick_file() {
        path
    } else {
        PathBuf::new()
    }
}

pub fn save_file(path: &PathBuf, content: String) {
    if path.exists() {
        _=fs::write(path, content);
    }
}

pub fn read_file(path: &PathBuf) -> String {
    if let Ok(text) = fs::read_to_string(path) {
        text
    } else {
        String::new()
    }
}
