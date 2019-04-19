use futures::future::Future;

use telegrambot::api::SendMessage;
use telegrambot::TelegramBot;

mod config;

fn main() {
  TelegramBot::new(config::config()).unwrap()
    .on_text(|(api, vtm)| {
      let first_name = vtm.message.from.unwrap().first_name;
      println!("<{}>: {}", first_name, vtm.text);
      telegrambot::spawn(
        api.send_message(&SendMessage::new(vtm.message.chat.id(), format!("Hi, {}! You just wrote '{}'", first_name, vtm.text)))
          .map(|_| {})
          .map_err(|_| {})
      );
    })
    .start()
    .unwrap();
}

