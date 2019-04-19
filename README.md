Telegrambot
===

[![Build Status](https://drone.0u0.me/api/badges/fewensa/telegrambot/status.svg)](https://drone.0u0.me/fewensa/telegrambot)


A [telegram bot api](https://core.telegram.org/bots/api) for rust.


# Usage

```toml
telegrambot = "0.1"
```


# Example

## A simple example. ([example/simple](./examples/simple.rs))


```rust
use futures::future::Future;

use telegrambot::api::SendMessage;
use telegrambot::TelegramBot;

fn main() {
  let token = env::var("TELEGRAM_BOT_TOKEN").unwrap();
  let cfg = Config::builder(token)
    .proxy("http://127.0.0.1:1081")
    .build()
    .unwrap();

  TelegramBot::new(cfg).unwrap()
    .on_text(|(api, vtm)| {
      let first_name = vtm.message.from.unwrap().first_name;
      println!("<{}>: {}", first_name, vtm.text);
      telegrambot::spawn(
        api.send_message(&SendMessage::new(vtm.message.chat.id(),
         format!("Hi, {}! You just wrote '{}'", first_name, vtm.text)))
          .map(|_| {})
          .map_err(|_| {})
      );
    })
    .start()
    .unwrap();
}
```

## A Command example ([example/command.rs](./example/command.rs))


```rust
use futures::future::Future;

use telegrambot::api::SendMessage;
use telegrambot::TelegramBot;

fn main() {
  let token = env::var("TELEGRAM_BOT_TOKEN").unwrap();
  let cfg = Config::builder(token)
    .proxy("http://127.0.0.1:1081")
    .build()
    .unwrap();

  TelegramBot::new(cfg).unwrap()
    .on_command("/list", |(api, vcm)| {
      telegrambot::spawn(
        api.send_message(&SendMessage::new(vcm.message.chat.id(),
         format!("Call list command")))
          .map(|_| {})
          .map_err(|_| {})
      );
    })
    .on_command("/page", |(api, vcm)| {
      telegrambot::spawn(
        api.send_message(&SendMessage::new(vcm.message.chat.id(),
         format!("Call page command, and arguments for this command: {:?}", vcm.args)))
          .map(|_| {})
          .map_err(|_| {})
      );
    })
    .start()
    .unwrap();
}
```

Support for parameter analysis. Some example: 

- `/page 1 10`
  
  args: ["1", "10"]
  
- `/start "Just do it" 'from now on'`

  args: ["Just do it", "from now on"]

- `/run 'Just do it'`

  args: ["Just do it"]

## More

More example you can see [examples](./examples)

## Other

Thanks to [telegram-rs/telegram-bot](https://github.com/telegram-rs/telegram-bot) project, telegram is a modification to `telegram-rs/telegram-bot`

