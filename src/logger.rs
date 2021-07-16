use crate::file_handling::functions::*;

use serde::Deserialize;

use std::process::Command;
use telegram_bot::*;

#[derive(Deserialize, Debug)]
pub struct Logger {
    created_at: String,
    id: String,
    logplex_url: String,
    updated_at: String,
}

pub fn run(telegram_api: &Api, chat: ChatId) {
    // This script generates jsons that contain
    // logger url using Heroku API.
    let logger_script = Command::new("bash")
                            .arg("scripts/logger.sh")
                            .output()
                            .expect("Script could not be found.");
    println!("logger script: {:?}", logger_script);

    let log_worker = get_logger_from_json("worker_log_details.json").unwrap();
    let log_api = get_logger_from_json("api_log_details.json").unwrap();
    let log_heroku_worker = get_logger_from_json("heroku_worker_log_details.json").unwrap();

    // Send logs of last bot runtime logs
    // Delete any old log files
    delete_file("log.txt".to_string());
    create_file("log.txt".to_string());

    let last_worker_log = Command::new("curl")
                            .arg(log_worker.logplex_url)
                            .args(&["--max-time", "2"])
                            .output()
                            .expect("log_worker could not be reached OR curl could not be run.");
    let last_api_log = Command::new("curl")
                        .arg(log_api.logplex_url)
                        .args(&["--max-time", "2"])
                        .output()
                        .expect("log_api could not be reached OR curl could not be run.");
    let last_heroku_worker_log = Command::new("curl")
                                    .arg(log_heroku_worker.logplex_url)
                                    .args(&["--max-time", "2"])
                                    .output()
                                    .expect("log_heroku_worker could not be reached OR curl could not be run.");
    let mut formatted_log = format!("{:?}\n{:?}\n{:?}",  last_worker_log, last_api_log, last_heroku_worker_log);

    // replace all \" with "
    formatted_log = str::replace(&formatted_log, "\\\"", "\"");
    // replace all \n with actual newlines
    formatted_log = str::replace(&formatted_log, "\\n", "\n");

    write_to_file("log.txt".to_string(), formatted_log.as_str());
    telegram_api.spawn(chat
        .document(&InputFileUpload::with_path("log.txt"))
        .caption("log from previous execution")
    );

}
