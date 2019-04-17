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
    .on_text(|vtex| {
      if let Some(reply) = &vtex.message.reply_to_message {
        reply.on_text(|vtex| {
          println!("<<<<<=====>>>> replay text message {:?}", vtex);
        })
          .on_sticker(|vtex| {
            println!("<<<<<=====>>>> replay sticker message {:?}", vtex);
          });
      }
      println!("=====> TEXT: {:?}", vtex);
    })
    .on_sticker(|sti| {
      println!("=====> STICKER: {:?}", sti);
    })
    .on_photo(|pho| {
      println!("=====> PHOTO: {:?}", pho);
    })
    .on_document(|doc| {
      println!("=====> DOCUMENT: {:?}", doc);
    })
    .on_callback_query(|cq| {
      println!("=====> DOCUMENT: {:?}", cq);
    })
    .on_command("/start",|cmd| {
      println!("=====> COMMAND /start  {:?}", cmd);
    })
    .on_command("/list",|cmd| {
      println!("=====> COMMAND /list  {:?}", cmd);
    })
    .start()
    .unwrap();
}
