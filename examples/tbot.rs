use std::env;

use telegrambot::api::{GetFile, SendMessage, BotApi};
use telegrambot::config::Config;
use telegrambot::TelegramBot;
use telegrambot::types::{ToChatRef, ParseMode};
use futures::future::{Future, IntoFuture};
use telegrambot::api::rawreq::RawReq;
use std::sync::Arc;

fn main() {
  let token = env::var("TELEGRAM_BOT_TOKEN").unwrap();
//  let cfg = Config::new(token);
//  let cfg = Config { token, mode: ConnectMode::Polling };
  let a = token.clone();
  let cfg = Config::builder(token)
//    .proxy("http://127.0.0.1:1081")
    .build()
    .unwrap();

  let rawreq = RawReq::new(Arc::new(cfg.client().clone()), a);
  let botapi = Arc::new(BotApi::new(rawreq));

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
    .on_sticker(move |(api, sti)| {
      println!("=====> STICKER: {:?} ===> FILEID: {:?}", sti, sti.sticker.file_id);
      let thumbs = &sti.sticker.thumb;
      println!("THUMBS: {:?}", thumbs);
//      let futapi = botapi.futapi();

      let fut = botapi.futapi()
        .get_file(&GetFile::new(thumbs.clone().unwrap()));
      let fut = fut.and_then(|file| botapi.futapi().send_message(
            SendMessage::new(sti.message.chat.clone(), file.unwrap().file_id)
              .parse_mode(ParseMode::Markdown)
          ));
      let fut = fut.map(|a| println!("{:?}", a))
        .map_err(|e| eprintln!("{:?}", e));
      tokio::spawn(fut);

//      let (tx, rx) = futures::sync::oneshot::channel();
//
//      api.futapi()
//        .get_file(&GetFile::new(thumbs.clone().unwrap()))
//        .select(api.futapi()
//          .get_file(&GetFile::new(thumbs.clone().unwrap())))
//        .map(|a|{})
//        .map_err(|e|{});
//
//      tokio::spawn(api.futapi()
//        .get_file(&GetFile::new(thumbs.clone().unwrap()))
//        .map(|file| {
//          println!("+=========> {:?}", file);
//          tx.send(file);
//        })
//        .map_err(|e| eprintln!("{:?}", e))
//      );
//
//
//      let a = rx.map(|file|{
//        println!("got: {:?}", file);
//        let fut = api.futapi().send_message(
//            SendMessage::new(sti.message.chat.clone(), file.unwrap().file_id)
//              .parse_mode(ParseMode::Markdown)
//          )
//          .map(|a| {})
//          .map_err(|e| eprintln!("{:?}", e));
//        tokio::spawn(fut);
//      }).wait();


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
//      let result1 = api.get_me();
      api.futapi().spawn(
        api.futapi()
          .send_message(SendMessage::new(cmd.message.chat.clone(), "*Hello*")
            .parse_mode(ParseMode::Markdown))
          .join(api.futapi().get_me())
          .map(|(a, b)| {})
          .map_err(|e| {})
      );
      println!("=====> COMMAND /list  {:?}", cmd);
    })
    .start()
    .unwrap();
}
