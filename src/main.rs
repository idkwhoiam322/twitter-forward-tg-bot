mod users;
mod creds;
use creds::credentials::*;
mod file_handling;
mod storage;
mod logger;
mod twitter;
use twitter::manage_tweets::send_tweets;

use teloxide::prelude::*;
use chrono::prelude::*;

async fn run() {
    let tg_bot = get_telegram_bot();

    let chat_id:i64 = -1001527066155; // test chat

    let utc_time: DateTime<Utc> = chrono::Utc::now();
    let startpost_text = format!("Starting bot at {}-{}-{} {}:{}:{} UTC.",
                utc_time.year(), utc_time.month(), utc_time.day(),
                utc_time.hour(), utc_time.minute(), utc_time.second());

    tg_bot.send_message(chat_id, startpost_text)
        .send()
        .await
        .expect("Message could not be sent");

    logger::run(&tg_bot, chat_id).await;

    send_tweets(tg_bot).await;
}

#[tokio::main]
async fn main() {
    run().await;
}
