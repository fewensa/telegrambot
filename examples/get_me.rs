use futures::future::Future;

use telegrambot::api::BotApi;

mod config;

fn main() {
  let config = config::config();
  let api = BotApi::new(config);
  tokio::run(futures::lazy(move || {
    tokio::spawn(api.get_me().map(|user| println!("{:?}", user)).map_err(|_| {}))
  }))
}

