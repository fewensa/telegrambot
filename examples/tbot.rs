use futures::future::Future;

use telegrambot::api::{BotApi, GetFile, SendMessage};
use telegrambot::TelegramBot;
use telegrambot::types::{ParseMode, PhotoSize};

mod config;

fn main() {
  TelegramBot::new(config::config()).unwrap()
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

      let fileid = sti.sticker.thumb.clone().unwrap().file_id;
      let chat = sti.message.chat.id();

      telegrambot::spawn(api.get_file(&GetFile::new(fileid))
        .and_then(move |file| {
          println!("{:?}", file);
          api.send_message(
            SendMessage::new(chat, file.unwrap().file_id)
              .parse_mode(ParseMode::Markdown))
        })
        .map(|a| println!("{:?}", a))
        .map_err(|e| eprintln!("{:?}", e)));
    })
    .on_video(|(api, vvm)| {
      let chat = vvm.message.chat.id();
      tokio::spawn(
        api.get_file(&GetFile::new(vvm.video.file_id.clone()))
          .and_then(move |file| {
            println!("{:?}", file);
            api.send_message(
              SendMessage::new(chat, file.unwrap().file_id)
                .parse_mode(ParseMode::Markdown))
          })
          .map(|a| println!("{:?}", a))
          .map_err(|e| eprintln!("{:?}", e))
      );
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
      telegrambot::api::spawn(api.send_message(SendMessage::new(cmd.message.chat.clone().id(), "*Hello*")
        .parse_mode(ParseMode::Markdown))
        .join(api.get_me())
        .map(|(a, b)| {})
        .map_err(|e| {}));
      println!("=====> COMMAND /list  {:?}", cmd);
    })
    .start()
    .unwrap();
}
