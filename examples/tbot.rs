use std::env;

use rbotele::{Config, ConnectMode, TelegramBot};

fn main() {
  let token = env::var("TELEGRAM_BOT_TOKEN").unwrap();
//  let cfg = Config::new(token);
  let cfg = Config { token, mode: ConnectMode::Polling };
  let bot = TelegramBot::new(cfg).unwrap()
    .command()
    .start()
    .unwrap();
}
