use crate::file_handling::functions::*;

use serde::Deserialize;

use std::process::Command;
use teloxide::prelude::*;
use teloxide::types::InputFile;
use curl::easy::{Easy, List};
use std::env;
use std::io::Read;

#[derive(Deserialize, Debug)]
pub struct Logger {
    created_at: String,
    id: String,
    logplex_url: String,
    updated_at: String,
}

pub async fn run(tg_bot: &Bot, chat_id: i64) {
    // Common Heroku data
    let mut list = List::new();
    list.append("Content-Type: application/json").unwrap();
    list.append("Accept: application/vnd.heroku+json; version=3").unwrap();
    let heroku_api_key = env::var("HEROKU_API_KEY").expect("set HEROKU_API_KEY, thank you");
    let heroku_api_header = format!("Authorization: Bearer {}", heroku_api_key) ;
    list.append(heroku_api_header.as_str()).unwrap();
    let heroku_url = "https://api.heroku.com/apps/twitter-forward-tg-bot/log-sessions";

    let file_name = "log_request_details.json";
    let mut heroku_data = "{\"lines\": 1500}".as_bytes();

    let mut temp_data = Vec::new();
    let mut easy = Easy::new();
    easy.url(heroku_url).unwrap();
    easy.post(true).unwrap();
    easy.post_field_size(heroku_data.len() as u64).unwrap();
    // Add custom headers
    easy.http_headers(list).unwrap();
    let mut transfer = easy.transfer();
    transfer.read_function(|data| {
        Ok(heroku_data.read(data).unwrap_or(0))
    }).unwrap();
    transfer.write_function(|data| {
        temp_data.extend_from_slice(data);
        let api_data = match std::str::from_utf8(&temp_data) {
            Ok(value) => value,
            Err(_) => "",
        };
        delete_file(file_name);
        create_file(file_name);
        write_to_file(file_name, api_data);
        Ok(data.len())
    }).unwrap();
    transfer.perform().unwrap();

    let log_worker = get_logger_from_json(file_name).unwrap();

    // Send logs of last bot runtime logs
    // Delete any old log files
    delete_file("log.txt");
    create_file("log.txt");

    let mut formatted_log = match std::str::from_utf8(
                                &Command::new("curl")
                                    .arg(log_worker.logplex_url)
                                    .output()
                                    .expect("log_worker could not be reached OR curl could not be run.")
                                    .stdout
                            ) {
                                Ok(value) => format!("{}", value),
                                _ => String::from("No log found.")
                            };

    // replace all \" with "
    formatted_log = str::replace(&formatted_log, "\\\"", "\"");
    // replace all \n with actual newlines
    formatted_log = str::replace(&formatted_log, "\\n", "\n");

    write_to_file("log.txt", formatted_log.as_str());

    tg_bot.send_document(chat_id, InputFile::File(std::path::PathBuf::from("log.txt")))
        .send()
        .await
        .expect("Document could not be sent");
}
