use std::env;

use rbotele::{Config, ConfigBuilder, ConnectMode, TelegramBot};

fn main() {
  let token = env::var("TELEGRAM_BOT_TOKEN").unwrap();
//  let cfg = Config::new(token);
//  let cfg = Config { token, mode: ConnectMode::Polling };
  let cfg = Config::builder(token)
    .proxy("http://localhost:1081")
    .build()
    .unwrap();
  let bot = TelegramBot::new(cfg).unwrap()
    .command()
    .start()
    .unwrap();
}
