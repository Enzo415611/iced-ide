use std::{fs, path::PathBuf};

use rfd::FileDialog;

pub fn open_file() -> PathBuf {
    if let Some(path) = FileDialog::new().pick_file() {
        path
    } else {
        PathBuf::new()
    }
}


pub fn read_file(path: &PathBuf) -> String {
    if let Ok(text) = fs::read_to_string(path) {
        text
    } else {
        String::new()
    }
}