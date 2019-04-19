use futures::future::Future;

use telegrambot::api::SendMessage;
use telegrambot::TelegramBot;

mod config;

fn main() {
  TelegramBot::new(config::config()).unwrap()
    .on_command("/list", |(api, vcm)| {
      telegrambot::spawn(
        api.send_message(&SendMessage::new(vcm.message.chat.id(), format!("Call list command")))
          .map(|_| {})
          .map_err(|_| {})
      );
    })
    .on_command("/page", |(api, vcm)| {
      telegrambot::spawn(
        api.send_message(&SendMessage::new(vcm.message.chat.id(), format!("Call page command, and arguments for this command: {:?}", vcm.args)))
          .map(|_| {})
          .map_err(|_| {})
      );
    })
    .start()
    .unwrap();
}

