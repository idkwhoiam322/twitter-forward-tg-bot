use crate::users::LIST_OF_USERS;
use crate::file_handling::functions::*;

pub fn store_latest_tweet(tweet: &egg_mode::tweet::Tweet, is_retweet: bool) {
    let file_name = String::from("latest_tweet.txt");
    let mut do_not_skip = false;

    // Skip if not replying to same user, ie. if it is not a thread
    // We do not want to share replies that are just thank yous and such.
    if let (Some(ref user), Some(ref screen_name)) =
        (tweet.user.as_ref(), tweet.in_reply_to_screen_name.as_ref()) {
        // There may be a case where LIST_OF_USERS may reply to someone else
        // but may contain information that they want to share.
        // If such a tweet is retweeted, we want this to be forwarded.
        // Post by someone else
        // |
        // |
        // ---- Post by LIST_OF_USERS ( id_1234 ) <-- Post will not be shared
        // If id_1234 is then retweeted, it will be shared.
        // TL;DR: Post retweeted replies.
        if is_retweet {
            do_not_skip = true;
        }
        // Replying to someone that isn't the original thread starter
        if user.screen_name.ne(&screen_name.to_string()) && !do_not_skip {
            return;
        }
    }

    // There can be instances where one twitter account might retweet
    // a post from another one, say @PlayVALORANT retweets a post from
    // @ValorantEsports. Our bot already tracks posts from @ValorantEsports
    // so it is unnecessary to share the retweet as well.
    for user in &tweet.entities.user_mentions {
        for cur_user in LIST_OF_USERS {
            if user.screen_name.eq(cur_user) && is_retweet && !do_not_skip {
                return;
            }
        }
    }

    if let Some(ref status) = tweet.retweeted_status {
        store_latest_tweet(status, true);
        return;
    }

    if let Some(ref user) = tweet.user {
        let formatted_entry = format!(
            "<a href='https://twitter.com/{tw_screen_name}/status/{tw_id}'>Tweet Source</a>\n\
            {tw_username} (<a href='https://twitter.com/{tw_screen_name}'>@{tw_screen_name}</a>):",
            tw_screen_name = &user.screen_name,
            tw_username = &user.name,
            tw_id = tweet.id
        );
        write_to_file(file_name.clone(), formatted_entry.as_str());
    }

    if let Some(ref _screen_name) = tweet.in_reply_to_screen_name {
        let formatted_entry = format!("➜ Thread reply:");
        write_to_file(file_name.clone(), formatted_entry.as_str());
    }

    let formatted_entry = format!("{}", &tweet.text);
    write_to_file(file_name.clone(), formatted_entry.as_str());

    if let Some(ref status) = tweet.quoted_status {
        let formatted_entry = format!("{}","➜ Quoting the following status:");
        write_to_file(file_name.clone(), formatted_entry.as_str());
        store_latest_tweet(status, false);
    }
}
