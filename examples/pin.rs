use futures::future::Future;

use telegrambot::api::{SendMessage, PinChatMessage, UnpinChatMessage};
use telegrambot::TelegramBot;
use telegrambot::vision::VMessageChat;

mod config;

fn main() {
  TelegramBot::new(config::config()).unwrap()
    .on_command("/pin", |(api, vcm)| {
      println!("incomming: {:?}", vcm);
      println!("is group: {:?}", vcm.message.is_group());
      if !vcm.message.is_supergroup() {
        return;
      }
      vcm.message.reply_to_message.map(|possibility| {
        possibility.with_message(|msg| {
          tokio::spawn(api.pin_chat_message(&PinChatMessage::new(msg.chat.id(), msg.id))
            .map(|_|{})
            .map_err(|_|{}));
        });
      });
    })
    .on_command("/unpin", |(api, vcm)| {
      if !vcm.message.is_supergroup() {
        return
      }
      telegrambot::spawn(api.unpin_chat_message(&UnpinChatMessage::new(vcm.message.chat.id()))
        .map(|_|{})
        .map_err(|_|{}));
    })
    .start()
    .unwrap();
}

