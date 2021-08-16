use crate::users::LIST_OF_USERS;
use crate::creds::credentials::*;
use crate::file_handling::functions::*;
use crate::storage::store_latest_tweet;

use std::error::Error;
use std::io::Read;
use egg_mode::user;
use teloxide::prelude::*;
use teloxide::types::ParseMode;
use std::{thread, time};
use regex::Regex;

pub static FILE_NAME: &str = "latest_tweet.txt";

fn unshorten_tco(latest_tweet: &String) -> String {
    let mut new_tweet = latest_tweet.clone();
    if latest_tweet.contains("https://t.co/") {
        for mat in Regex::new(r"\bhttps://t\.co/[a-zA-Z0-9]*\b").unwrap().find_iter(&latest_tweet) {
            let url = &latest_tweet[mat.start()..mat.end()];
            log::info!("old url: {:?}", url);
            match urlexpand::unshorten(&url, None) {
                Some(new_url) => {
                    new_tweet = str::replace(&new_tweet, url, &new_url);
                    log::info!("new url: {:?}", new_url);
                }
                None =>  log::debug!("URL {:?} could not be expanded.", url),
            };
        }
    }
    new_tweet
}

pub async fn send_tweets(tg_bot: Bot) -> Result<(), Box<dyn Error>> {
    let twitter_token = get_twitter_token();

    let sleep_time = time::Duration::from_millis(1000);

    const TOTAL_USERS:usize = LIST_OF_USERS.len();
    // initialize blank id array for tweets to prevent reposting
    let mut prev_id: [u64; TOTAL_USERS] = [0; TOTAL_USERS];
    let mut users_iter = 0;
    let mut total_iter:u64 = 0;

    // https://t.me/PlayVALORANT_tweets
    let chat_id:i64 = -1001512385809;

    // We don't post the first set of tweets to channel to prevent reposts
    // in case of a bot update or a dyno cycle
    let mut skip = 0;

    // LOOP FROM HERE
    'outer: loop {
        let target_user = user::UserID::ScreenName(LIST_OF_USERS[users_iter].into());

        // Delete any old files
        delete_file(FILE_NAME);

        // create new file to store latest tweet
        let mut latest_tweet_file = create_file(FILE_NAME);

        let f = egg_mode::tweet::user_timeline::<user::UserID>(target_user, true, true, &twitter_token);
        let feed = f.start().await?.1;

        for status in feed.iter() {
            if  status.id == prev_id[users_iter] {
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
            if let Some(user) = &status.user {
                log::info!("\nNew post from: {:?}: https://twitter.com/{tw_screen_name}/status/{tw_id}",
                    LIST_OF_USERS[users_iter],
                    tw_screen_name = &user.screen_name,
                    tw_id = &status.id);
            }

            store_latest_tweet(&status, false);
        }

        // Save latest tweet from file to a string
        let mut latest_tweet = String::new();
        latest_tweet_file.read_to_string(&mut latest_tweet)
            .expect("File could not be read.");

        // Don't post the first set of tweets to channel to prevent reposts
        if total_iter < TOTAL_USERS as u64 {
           skip = 1;
        }

        // Expand each t.co url
        let latest_tweet = tokio::task::spawn_blocking(move || {  // move is needed for:  ^^ may outlive borrowed value `latest_tweet`
                                unshorten_tco(&latest_tweet)
                            })
                            .await
                            .expect("Thread panicked");
        log::info!("Final Tweet:\n{:?}", latest_tweet);

        // Do not attempt to post empty messages
        // This will happen in instances such as when we have a tweet that is replying to
        // another user.
        // Don't post the first set of tweets to channel to prevent reposts
        if latest_tweet.to_string().ne("") && skip == 0 {
            tg_bot.send_message(chat_id, latest_tweet.to_string())
                .parse_mode(ParseMode::Html)
                .disable_web_page_preview(true)
                .send()
                .await
                .log_on_error()
                .await;
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

        // reset skip
        skip = 0;
    }
    // LOOP TILL HERE
}
