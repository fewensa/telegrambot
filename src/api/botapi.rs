use std::sync::Arc;
use crate::config::Config;
use crate::api::get_me::GetMe;
use crate::api::req::TGReq;
use futures::future::Future;
use crate::api::TGResp;
use crate::errors::TGBotError;
use crate::tglog;
use reqwest::r#async::{Client, Decoder};
use std::{mem, io};
use futures::stream::Stream;
use std::io::Cursor;

pub struct BotApi {
  cfg: Arc<Config>,
}

impl BotApi {
  pub fn new(cfg: Arc<Config>) -> Self {
    BotApi {
      cfg
    }
  }

  pub fn get_me(&self) {
    let gm = GetMe;
//    let fut = gm.request(self.cfg.clone())
//      .map(|resp| {
////        let dez: Result<<Req::Resp as TGResp>::Type, TGBotError> = Req::Resp::deserialize(resp);
////        match dez {
////          Ok(ret) => {
////            debug!(tglog::telegram(), "GET_ME: {:?}", ret);
////            Ok(())
////          },
////          Err(err) => {
////            error!(tglog::telegram(), "Call telegram api fail: {:?}", err);
////            Ok(())
////          }
////        }
//        debug!(tglog::telegram(  ), "GET_ME: {:?}", resp);
//        Ok(())
//      }).map_err(|e| Err(e));
    tokio::run(fetch());
  }


}

fn fetch() -> impl Future<Item=(), Error=()> {
  Client::new()
    .get("https://hyper.rs")
    .send()
    .and_then(|mut res| {
      println!("{}", res.status());

      let body = mem::replace(res.body_mut(), Decoder::empty());
      body.concat2()
    })
    .map_err(|err| println!("request error: {}", err))
    .map(|body| {
      let mut body = Cursor::new(body);
      let _ = io::copy(&mut body, &mut io::stdout())
        .map_err(|err| {
          println!("stdout error: {}", err);
        });
    })
}
