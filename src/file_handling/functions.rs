use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;

fn open_file(name: String) -> std::fs::File {
    let file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(name.as_str())
        .unwrap();
    file
}

pub fn create_file(name: String) -> std::fs::File {
    let file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .read(true)
        .open(name.as_str())
        .expect("File could not be created.");
    file
}

pub fn delete_file(name: String) {
    if Path::new(name.as_str()).exists() {
        std::fs::remove_file(name.as_str())
            .expect("File could not be deleted.");
    }
}

pub fn write_to_file(file_name: String, entry: &str) {
    let mut file = open_file(file_name);
    writeln!(file, "{}", entry)
        .expect("File could not be written into.");
}
