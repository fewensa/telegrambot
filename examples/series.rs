use futures::future::Future;

use telegrambot::api::{GetFile, PinChatMessage, SendMessage, UnpinChatMessage};
use telegrambot::TelegramBot;
use telegrambot::types::ParseMode;
use telegrambot::vision::VMessageChat;

mod config;

fn main() {
  TelegramBot::new(config::config()).unwrap()
    .on_sticker(|(api, vsm)| {
      let sticker = vsm.sticker;
      let chat = vsm.message.chat.id();

      println!("stick emoji: {:?}", sticker.clone().emoji.unwrap_or("None".to_string()));
      telegrambot::spawn(
        api.get_file(&GetFile::new(sticker.clone().thumb.unwrap().file_id))
          .and_then(move |file| api.send_message(
            SendMessage::new(chat, format!("filesize *{:?}* and id is *{:?}*", sticker.clone().file_size.unwrap_or(-1), file.unwrap().file_id))
              .parse_mode(ParseMode::Markdown)))
          .map(|a| println!("{:?}", a))
          .map_err(|e| eprintln!("{:?}", e))
      );
    })
    .start()
    .unwrap();
}

