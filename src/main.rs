mod users;
mod creds;
use creds::credentials::*;
mod file_handling;
mod storage;
mod logger;
mod twitter;
use twitter::manage_tweets::send_tweets;

use std::error::Error;
use teloxide::prelude::*;
use teloxide::types::{
    ParseMode, Me,
};
use chrono::prelude::*;

async fn run() -> Result<(), Box<dyn Error>> {
    let tg_bot = get_telegram_bot();

    let chat_id:i64 = -1001527066155; // test chat

    let Me { user: bot_user, .. } = tg_bot.get_me().send().await.unwrap();
    let bot_name = bot_user.username.expect("Bots must have usernames");
    let utc_time: DateTime<Utc> = chrono::Utc::now();
    let startpost_text = format!("Starting @{} at <code>{}-{}-{} {}:{}:{} UTC</code>.",
                            bot_name,
                            utc_time.year(), utc_time.month(), utc_time.day(),
                            utc_time.hour(), utc_time.minute(), utc_time.second());

    tg_bot.send_message(chat_id, startpost_text)
        .parse_mode(ParseMode::Html)
        .send()
        .await
        .expect("Message could not be sent");

    logger::run(&tg_bot, chat_id).await;

    send_tweets(tg_bot).await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    run().await.unwrap();
    Ok(())
}
