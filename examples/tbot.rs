use std::env;

use rbotele::{Config, ConfigBuilder, ConnectMode, TelegramBot, Track};

fn main() {
  let token = env::var("TELEGRAM_BOT_TOKEN").unwrap();
//  let cfg = Config::new(token);
//  let cfg = Config { token, mode: ConnectMode::Polling };
  let cfg = Config::builder(token)
//    .proxy("http://127.0.0.1:1081")
    .build()
    .unwrap();
  TelegramBot::new(cfg).unwrap()
    .on_text(Track::All)
    .on_update()
    .start()
    .unwrap();
}
