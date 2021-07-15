use std::env;

// Set Telegram Bot Token from @BotFather
pub fn get_telegram_bot_token() -> String {
    let tg_bot_token = env::var("TELEGRAM_BOT_TOKEN")
            .expect("set TELEGRAM_BOT_TOKEN, thank you");
    tg_bot_token
}

// Set Twitter API keys
pub fn get_twitter_token() -> egg_mode::Token {
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

    twitter_token
}
