use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;
use std::fs::File;
use std::error;
use std::io::BufReader;

use crate::logger::Logger;

pub fn get_logger_from_json<P: AsRef<Path>>(path: P) -> Result<Logger, Box<dyn error::Error>> {
    // Open the file in read-only mode with buffer.
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `Logger`.
    let logger = serde_json::from_reader(reader)?;

    // Return the `Logger`.
    Ok(logger)
}

pub fn open_file(name: &str) -> std::fs::File {
    let file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(name)
        .unwrap();
    file
}

pub fn create_file(name: &str) -> std::fs::File {
    let file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .read(true)
        .open(name)
        .expect("File could not be created.");
    file
}

pub fn delete_file(name: &str) {
    if Path::new(name).exists() {
        std::fs::remove_file(name)
            .expect("File could not be deleted.");
    }
}

pub fn write_to_file(file_name: &str, entry: &str) {
    let mut file = open_file(file_name);
    writeln!(file, "{}", entry)
        .expect("File could not be written into.");
}
