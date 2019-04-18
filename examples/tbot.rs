use std::env;

use telegrambot::api::{GetFile, SendMessage};
use telegrambot::config::Config;
use telegrambot::TelegramBot;
use telegrambot::types::{ToChatRef, ParseMode};
use futures::future::{Future, IntoFuture};

fn main() {
  let token = env::var("TELEGRAM_BOT_TOKEN").unwrap();
//  let cfg = Config::new(token);
//  let cfg = Config { token, mode: ConnectMode::Polling };
  let cfg = Config::builder(token)
//    .proxy("http://127.0.0.1:1081")
    .build()
    .unwrap();


  TelegramBot::new(cfg).unwrap()
    .on_text(|(api, vtex)| {
      if let Some(reply) = &vtex.message.reply_to_message {
        reply.with_text(|vtex| {
          println!("<<<<<=====>>>> replay text message {:?}", vtex);
        })
          .with_sticker(|vtex| {
            println!("<<<<<=====>>>> replay sticker message {:?}", vtex);
          });
      }
      println!("=====> TEXT: {:?}", vtex);
    })
    .on_sticker(|(api, sti)| {
      println!("=====> STICKER: {:?} ===> FILEID: {:?}", sti, sti.sticker.file_id);
      let thumbs = &sti.sticker.thumb;
      println!("THUMBS: {:?}", thumbs);
      let chat = sti.message.chat.to_chat_ref();
      let result = api.get_file(&GetFile::new(thumbs.clone().unwrap()));

      if let Ok(f) = result {
        api.send_message(&SendMessage::new(chat, f.unwrap().file_id));
      }
    })
    .on_photo(|(api, pho)| {
      println!("=====> PHOTO: {:?}", pho);
    })
    .on_document(|(api, doc)| {
      println!("=====> DOCUMENT: {:?}", doc);
    })
    .on_callback_query(|(api, cq)| {
      println!("=====> DOCUMENT: {:?}", cq);
    })
    .on_command("/start", |(api, cmd)| {
      println!("=====> COMMAND /start  {:?}", cmd);
    })
    .on_command("/list", |(api, cmd)| {
      if cmd.message.is_edited {
        return;
      }
      let result1 = api.get_me();
      api.futapi().spawn(
        api.futapi().send_message(SendMessage::new(cmd.message.chat.clone(), "*Hello*").parse_mode(ParseMode::Markdown))
          .map(|a| {})
          .map_err(|e| {})
      );
      println!("=====> COMMAND /list  {:?}", cmd);
    })
    .start()
    .unwrap();
}
