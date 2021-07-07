use std::io::Read;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;
use std::env;

use egg_mode::user;

use telegram_bot::*;

use std::{thread, time};

// Select user
// ValorantEsports - Use this for VCT
// PlayVALORANT - Official VALORANT account
// ValorLeaks - VALORANT leaks
// CheckValor - VALORANT update cheker
const LIST_OF_USERS: &'static [&'static str] =
    &[
        "ValorantEsports",
        "ValorLeaks",
        "CheckValor",
        "PlayVALORANT"
    ];

fn store_latest_tweet(tweet: &egg_mode::tweet::Tweet) {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("latest_tweet.txt")
        .unwrap();

    // Skip if not replying to same user, ie. if it is not a thread
    // We do not want to share replies that are just thank yous and such.
    if let (Some(ref user), Some(ref screen_name)) =
        (tweet.user.as_ref(), tweet.in_reply_to_screen_name.as_ref()) {
        if user.screen_name.ne(&screen_name.to_string()) {
            return;
        }
    }

    // There can be instances where one twitter account might retweet
    // a post from another one, say @PlayVALORANT retweets a post from
    // @ValorantEsports. Our bot already tracks posts from @ValorantEsports
    // so it is unnecessary to share the retweet as well.
    for user in &tweet.entities.user_mentions {
        for cur_user in LIST_OF_USERS {
            if user.screen_name.eq(cur_user) {
                return;
            }
        }
    }

    if let Some(ref user) = tweet.user {
        let formatted_entry = format!(
            "Link to tweet: https://twitter.com/{}/status/{}\n\
            Tweet Preview:\n{} (@{}) posted",
            &user.screen_name, tweet.id, &user.name, &user.screen_name
        );
        writeln!(file, "{}", formatted_entry.as_str())
            .expect("File could not be written into.");
    }

    if let Some(ref screen_name) = tweet.in_reply_to_screen_name {
        let formatted_entry = format!("➜ in reply to @{}", screen_name);
        writeln!(file, "{}", formatted_entry.as_str())
            .expect("File could not be written into.");
    }

    if let Some(ref status) = tweet.retweeted_status {
        let formatted_entry = format!("{}", "Retweet ➜");
        writeln!(file, "{}", formatted_entry.as_str())
            .expect("File could not be written into.");
        store_latest_tweet(status);
        return;
    } else {
        let formatted_entry = format!("{}", &tweet.text);
        writeln!(file, "{}", formatted_entry.as_str())
            .expect("File could not be written into.");
    }

    if let Some(ref place) = tweet.place {
        let formatted_entry = format!("➜ from: {}", place.full_name);
        writeln!(file, "{}", formatted_entry.as_str())
            .expect("File could not be written into.");
    }

    if let Some(ref status) = tweet.quoted_status {
        let formatted_entry = format!("{}","➜ Quoting the following status:");
        writeln!(file, "{}", formatted_entry.as_str())
            .expect("File could not be written into.");
        store_latest_tweet(status);
    }

    if let Some(ref media) = tweet.extended_entities {
        let formatted_entry = format!("➜ Media attached to the tweet:");
        writeln!(file, "{}", formatted_entry.as_str())
            .expect("File could not be written into.");
        for info in &media.media {
            let formatted_entry = format!("  A {:?}", info.media_type);
            writeln!(file, "{}", formatted_entry.as_str())
                .expect("File could not be written into.");
        }
    }
}

#[tokio::main]
async fn main() {
    let tg_bot_token = env::var("TELEGRAM_BOT_TOKEN")
                        .expect("set TELEGRAM_BOT_TOKEN, thank you");
    let api = Api::new(tg_bot_token);

    let con_api_key = env::var("CONSUMER_API_KEY")
                        .expect("set CONSUMER_API_KEY, thank you");
    let con_api_secret_key = env::var("CONSUMER_API_SECRET_KEY")
                                .expect("set CONSUMER_API_SECRET_KEY, thank you");
    let con_token = egg_mode::KeyPair::new(
        con_api_key,
        con_api_secret_key,
    );

    let access_key = env::var("ACCESS_KEY")
                        .expect("set ACCESS_KEY, thank you");
    let access_secret_key = env::var("ACCESS_SECRET_KEY")
                                .expect("set ACCESS_SECRET_KEY, thank you");
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

    const TOTAL_USERS:usize = LIST_OF_USERS.len();
    // initialize blank id array for tweets to prevent reposting
    let mut prev_id: [u64; TOTAL_USERS] = [0; TOTAL_USERS];
    let mut users_iter = 0;
    let mut total_iter:u64 = 0;

    // LOOP FROM HERE
    'outer: loop {
        // print empty line to give a gap after each iteration
        println!("");
        let target_user = user::UserID::ScreenName(LIST_OF_USERS[users_iter].into());
        println!("Iteration #{} for {:?}", total_iter, LIST_OF_USERS[users_iter]);
        if Path::new("latest_tweet.txt").exists() {
            // Delete any old files
            std::fs::remove_file("latest_tweet.txt")
                .expect("File could not be deleted.");
        }
        // initialize latest tweet struct
        let mut latest_tweet_file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open("latest_tweet.txt")
            .expect("File could not be created.");

        let f = egg_mode::tweet::user_timeline::<user::UserID>(target_user, true, true, &twitter_token);
        let (_f, feed) = f.start().await.unwrap();

        for status in feed.iter() {
            if  status.id == prev_id[users_iter] {
                println!("No new tweet found! Sleeping for {:?}.", sleep_time);
                thread::sleep(sleep_time);
                // user must be changed before we go to next loop
                // Check for next user
                if users_iter == TOTAL_USERS-1 {
                    users_iter = 0;
                } else {
                    users_iter = users_iter + 1;
                }
                continue 'outer;
            }
        }

        for status in feed.iter().take(1) {
            store_latest_tweet(&status);
        }

        // Save latest tweet from file to a string
        let mut latest_tweet = String::new();
        latest_tweet_file.read_to_string(&mut latest_tweet)
            .expect("File could not be read.");
        println!("{:?}", latest_tweet);


        let mut chat = ChatId::new(-1001512385809); // https://t.me/PlayVALORANT_tweets

        // Horrible workaround to not post repeated posts for now.
        // Any new posts during updating will be missed
        if total_iter < TOTAL_USERS as u64 {
            chat = ChatId::new(-540381478); // test chat
        }

        // Do not attempt to post empty messages
        // This will happen in instances such as when we have a tweet that is replying to
        // another user.
        if latest_tweet.to_string().ne("") {
            api.spawn(chat
                        .text(latest_tweet.to_string())
                    );
        }

        for status in feed.iter() {
            prev_id[users_iter] = status.id;
        }

        // Check for next user
        if users_iter == TOTAL_USERS-1 {
            users_iter = 0;
        } else {
            users_iter = users_iter + 1;
        }

        total_iter = total_iter + 1;
    }
    // LOOP TILL HERE
}
