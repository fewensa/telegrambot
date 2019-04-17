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
use crate::types::Update;

pub const TELEGRAM_API_URL: &'static str = "https://tgb.akafwtll.tk/";

pub struct BotApi {
  rawreq: RawReq,
}

impl BotApi {
  pub fn new(rawreq: RawReq) -> Self {
    BotApi {
      rawreq
    }
  }

  pub fn get_update(&self, get_updates: &GetUpdates) -> TGFuture<Option<Vec<Update>>> {
    send(&self.rawreq, get_updates)
//      .map(|item| item)
//      .map_err(|e| e)
  }

//  pub fn get_me(&self) {
////    let gm = GetMe;
//    tokio::run(fetch());
//  }
}

fn send<Req: TGReq>(rawreq: &RawReq, req: &Req) -> TGFuture<Option<<Req::Resp as TGResp>::Type>> {
  let request = futures::future::result(req.request());
  let response = request.and_then(|httpreq| {
    rawreq.request(httpreq)
  });
  let future = response.and_then(|httpresp| {
    // let dez: Result<<Req::Resp as TGResp>::Type, TGBotError> =
    Req::Resp::deserialize(httpresp)
  });
  TGFuture::new(Box::new(future))

//  let future = fetch()
//    .and_then(|a| {
//      Ok(Some(Vec::new()))
//    });
//
//  TGFuture::new(Box::new(future))
}

fn fetch() -> impl Future<Item=HttpResp, Error=TGBotError> {
  Client::new()
    .get("https://hyper.rs")
    .send()
    .and_then(|mut res| {
      println!("{}", res.status());

      let body = mem::replace(res.body_mut(), Decoder::empty());
      body.concat2()
    })
    .map_err(|err| TGBotErrorKind::Other.into_err())
    .and_then(|body| {
      Ok(HttpResp {
        body: Some(Vec::from(body.as_ref()))
      })
    })
}

