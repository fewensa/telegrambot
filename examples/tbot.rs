use std::env;

use rbotele::{api, TelegramBot};
use rbotele::config::Config;

fn main() {
  let token = env::var("TELEGRAM_BOT_TOKEN").unwrap();
//  let cfg = Config::new(token);
//  let cfg = Config { token, mode: ConnectMode::Polling };
  let cfg = Config::builder(token)
//    .proxy("http://127.0.0.1:1081")
    .build()
    .unwrap();


  TelegramBot::new(cfg)
    .unwrap()
//    .on_update(|update| {
//      println!("{:?}", update);
//    })
//    .on_text_message(|(message, edited)| {
//      match edited {
//        false => println!("POST {:#?}", message),
//        true => println!("EDITED {:#?}", message)
//      }
//      api::get_me();
//    })
    .on_text(|message| {
      println!("=====> TEXT: {:?}", message);
    })
    .on_sticker(|message| {
      println!("=====> STICKER: {:?}", message);
    })
    .on_photo(|message| {
      println!("=====> PHOTO: {:?}", message);
    })
    .on_document(|message| {
      println!("=====> DOCUMENT: {:?}", message);
    })
    .start()
    .unwrap();
}
