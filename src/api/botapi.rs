use std::{io, mem};
use std::io::Cursor;
use std::sync::Arc;

use error_chain_mini::ErrorKind;
use futures::future::Future;
use futures::stream::Stream;
use reqwest::r#async::{Client, Decoder};

use crate::api::get_updates::GetUpdates;
use crate::api::rawreq::RawReq;
use crate::api::req::{HttpReq, TGReq};
use crate::api::resp::HttpResp;
use crate::api::TGResp;
use crate::config::Config;
use crate::errors::{TGBotError, TGBotErrorKind};
use crate::tgfut::TGFuture;
use crate::tglog;
use crate::types::{Update, User};
use crate::api::get_me::GetMe;

pub const TELEGRAM_API_URL: &'static str = "https://tgb.akafwtll.tk/";

pub struct BotApi {
  futapi: BotFutApi,
}

impl BotApi {
  pub fn new(rawreq: RawReq) -> Self {
    let futapi = BotFutApi { rawreq };
    BotApi {
      futapi
    }
  }

  pub fn futapi(&self) -> &BotFutApi {
    &self.futapi
  }


  fn fnc_call<F, T>(&self, fnc: F, fut: TGFuture<Option<T>>)
    where F: Fn((Option<T>, Option<TGBotError>)) + Send + Sync + Clone + 'static,
          T: 'static {
    let fnc_map = fnc.clone();
    let fnc_map_err = fnc.clone();
    tokio::spawn(fut.map(move |item| fnc_map((item, None)))
      .map_err(move |e| fnc_map_err((None, Some(e)))));
  }

  pub fn get_me<F>(&self, fnc: F) where F: Fn((Option<User>, Option<TGBotError>)) + Send + Sync + Clone + 'static {
    self.fnc_call(fnc, self.futapi().get_me())
  }
}

pub struct BotFutApi {
  rawreq: RawReq,
}

impl BotFutApi {
  fn send<Req: TGReq>(&self, req: &Req) -> TGFuture<Option<<Req::Resp as TGResp>::Type>> {
    let request = futures::future::result(req.request());
    let rawreq = self.rawreq.clone();
    let response = request.and_then(move |httpreq| {
      rawreq.request(httpreq)
    });
    let future = response
      .map(move |resp| {
        let dez: Result<<Req::Resp as TGResp>::Type, TGBotError> = Req::Resp::deserialize(resp);
        match dez {
          Ok(ret) => Some(ret),
          Err(err) => {
            // todo: if error do more thing
            error!(tglog::telegram(), "Call telegram api fail: {:?}", err);
            None
          }
        }
      }).map_err(|e| e);
    TGFuture::new(Box::new(future))
  }

  pub fn get_update(&self, get_updates: &GetUpdates) -> TGFuture<Option<Vec<Update>>> {
    self.send(get_updates)
  }

  pub fn get_me(&self) -> TGFuture<Option<User>> {
    self.send(&GetMe)
  }
}



