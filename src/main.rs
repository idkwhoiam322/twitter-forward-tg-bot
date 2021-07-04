use std::io::Read;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;
use std::env;

use egg_mode::user;

use telegram_bot::*;
use std::time::Duration;

use std::{thread, time};

fn store_latest_tweet(tweet: &egg_mode::tweet::Tweet) {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("latest_tweet.txt")
        .unwrap();

    if let Some(ref user) = tweet.user {
        let formatted_entry = format!(
            "Link to tweet: https://twitter.com/{}/status/{}\n\
            Tweet Preview:\n{} (@{}) posted",
            &user.screen_name, tweet.id, &user.name, &user.screen_name
        );
        let entry_slice: &str = &formatted_entry[..];
        writeln!(file, "{}", entry_slice)
            .expect("File could not be written into.");
    }

    if let Some(ref screen_name) = tweet.in_reply_to_screen_name {
        let formatted_entry = format!("➜ in reply to @{}", screen_name);
        let entry_slice: &str = &formatted_entry[..];
        writeln!(file, "{}", entry_slice)
            .expect("File could not be written into.");
    }

    if let Some(ref status) = tweet.retweeted_status {
        let formatted_entry = format!("{}", "Retweet ➜");
        let entry_slice: &str = &formatted_entry[..];
        writeln!(file, "{}", entry_slice)
            .expect("File could not be written into.");
            store_latest_tweet(status);
        return;
    } else {
        let formatted_entry = format!("{}", &tweet.text);
        let entry_slice: &str = &formatted_entry[..];
        writeln!(file, "{}", entry_slice)
            .expect("File could not be written into.");
    }

    if let Some(ref place) = tweet.place {
        let formatted_entry = format!("➜ from: {}", place.full_name);
        let entry_slice: &str = &formatted_entry[..];
        writeln!(file, "{}", entry_slice)
            .expect("File could not be written into.");
    }

    if let Some(ref status) = tweet.quoted_status {
        let formatted_entry = format!("{}","➜ Quoting the following status:");
        let entry_slice: &str = &formatted_entry[..];
        writeln!(file, "{}", entry_slice)
            .expect("File could not be written into.");
            store_latest_tweet(status);
    }

    if let Some(ref media) = tweet.extended_entities {
        let formatted_entry = format!("➜ Media attached to the tweet:");
        let entry_slice: &str = &formatted_entry[..];
        writeln!(file, "{}", entry_slice)
            .expect("File could not be written into.");
        for info in &media.media {
            let formatted_entry = format!("  A {:?}", info.media_type);
            let entry_slice: &str = &formatted_entry[..];
            writeln!(file, "{}", entry_slice)
                .expect("File could not be written into.");
        }
    }
}

#[tokio::main]
async fn main() {
    let tg_bot_token = env::var("TELEGRAM_BOT_TOKEN").expect("set TELEGRAM_BOT_TOKEN, thank you");
    let api = Api::new(tg_bot_token);

    let con_api_key = env::var("CONSUMER_API_KEY").expect("set CONSUMER_API_KEY, thank you");
    let con_api_secret_key = env::var("CONSUMER_API_SECRET_KEY").expect("set CONSUMER_API_SECRET_KEY, thank you");
    let con_token = egg_mode::KeyPair::new(
        con_api_key,
        con_api_secret_key,
    );

    let access_key = env::var("ACCESS_KEY").expect("set ACCESS_KEY, thank you");
    let access_secret_key = env::var("ACCESS_SECRET_KEY").expect("set ACCESS_SECRET_KEY, thank you");
    let access_token = egg_mode::KeyPair::new(
        access_key,
        access_secret_key,
    );
    let twitter_token = egg_mode::Token::Access {
        consumer: con_token,
        access: access_token,
    };

    // 30s = 30*1000ms
    let sleep_time = time::Duration::from_millis(30000);
    let mut prev_id = 0u64;

    // LOOP FROM HERE
    'outer: loop {
        if Path::new("latest_tweet.txt").exists() {
            // Delete any old files
            std::fs::remove_file("latest_tweet.txt").expect("File could not be deleted.");
        }
        // initialize latest tweet struct
        let mut latest_tweet_file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open("latest_tweet.txt")
            .expect("File could not be created.");

        // Select user
        // ValorantEsports - Use this for VCT
        // PlayVALORANT
        // Prefer not to use multiple at a time to avoid recurring posts because of retweets
        let target_user = user::UserID::ScreenName("ValorantEsports".into());

        let f = egg_mode::tweet::user_timeline::<user::UserID>(target_user, true, true, &twitter_token);
        let (_f, feed) = f.start().await.unwrap();

        for status in feed.iter() {
            if  status.id == prev_id {
                println!("No new tweet found! Sleeping for {:?}.", sleep_time);
                thread::sleep(sleep_time);
                continue 'outer;
            }
        }

        for status in feed.iter() {
            store_latest_tweet(&status);
            println!("");
            break; // post latest only
        }

        // Save latest tweet from file to a string
        let mut latest_tweet = String::new();
        latest_tweet_file.read_to_string(&mut latest_tweet).expect("File could not be read.");
        println!("{:?}", latest_tweet);

        let chat = ChatId::new(-1001512385809); // https://t.me/PlayVALORANT_tweets
        api.send_timeout(chat.text(latest_tweet.to_string()), Duration::from_secs(1))
            .await
            .expect("Could not send message");
        
        for status in feed.iter() {
            prev_id = status.id;
        }
    }
    // LOOP TILL HERE
}
