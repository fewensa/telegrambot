use std::env;

use telegrambot::{api, TelegramBot};
use telegrambot::config::Config;

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
    .on_text(|message| {
      if let Some(reply) = &message.message.reply_to_message {
        reply.on_text(|vtm| {
          println!("<<<<<=====>>>> replay text message {:?}", vtm);
        })
          .on_sticker(|vsm| {
            println!("<<<<<=====>>>> replay sticker message {:?}", vsm);
          });
      }
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
