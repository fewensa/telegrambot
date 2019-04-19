use std::env;

use telegrambot::config::Config;

pub fn config() -> Config {
  let token = env::var("TELEGRAM_BOT_TOKEN").unwrap();
  Config::builder(token)
    .proxy("http://127.0.0.1:1081")
    .build()
    .unwrap()
}
