use std::collections::VecDeque;
use std::sync::Arc;
use std::time::Duration;

use error_chain_mini::ErrorKind;
use futures::{Async, Stream};
use futures::future::Future;
use tokio::timer::Interval;

use crate::config::Config;
use crate::errors::TGBotError;
use crate::errors::TGBotErrorKind;
use crate::tgfut::TGFuture;
use crate::tglog;
use crate::types::Update;
use crate::api::{rawreq, BotApi, TGReq, TGResp, GetUpdates};
use crate::api::rawreq::RawReq;

const TELEGRAM_LONG_POLL_TIMEOUT_SECONDS: i64 = 5;
const TELEGRAM_LONG_POLL_ERROR_DELAY_MILLISECONDS: u64 = 500;

pub struct UpdatesStream<'a> {
  //    interval: Interval,
  api: &'a BotApi,
  error_interval: Interval,
  last_update: i64,
  buffer: VecDeque<Update>,
  updates: Option<TGFuture<Option<Vec<Update>>>>,
}

impl<'a> UpdatesStream<'a> {
  pub fn new(api: &'a BotApi) -> Self {
    UpdatesStream {
      api,
//      interval: Interval::new_interval(Duration::from_secs(TELEGRAM_LONG_POLL_TIMEOUT_SECONDS)),
      error_interval: Interval::new_interval(Duration::from_millis(TELEGRAM_LONG_POLL_ERROR_DELAY_MILLISECONDS)),
      last_update: 0,
      buffer: VecDeque::new(),
      updates: None,
    }
  }
}

impl<'a> Stream for UpdatesStream<'a> {
  type Item = Update;
  type Error = TGBotError; // todo: do not return error, if happen error, wait and retry

  fn poll(&mut self) -> Result<Async<Option<Self::Item>>, Self::Error> {
    if let Some(value) = self.buffer.pop_front() {
      return Ok(Async::Ready(Some(value)));
    }

//    try_ready!(self.interval.poll().map_err(|_| TGBotErrorKind::Other.into_with(|| "Interval error")));

//    let cfg = self.cfg.clone();
    let last_update = self.last_update;

    let upfut = self.updates.get_or_insert_with(|| {
//      self::send(self.api, GetUpdates::new()
//        .offset(last_update + 1)
//        .timeout(TELEGRAM_LONG_POLL_TIMEOUT_SECONDS))
      self.api.get_update(GetUpdates::new()
        .offset(last_update + 1)
        .timeout(TELEGRAM_LONG_POLL_TIMEOUT_SECONDS))
    });

    let _updates: Option<Vec<Update>> = match upfut.poll() {
      Ok(Async::Ready(t)) => t,
      Ok(Async::NotReady) => return Ok(Async::NotReady),
      Err(err) => {
//        try_ready!(self.error_interval.poll().map_err(|_| TGBotErrorKind::Other.into_with(|| "Interval error")));
        self.updates = None;
        // todo: do not return error, if happen error, wait and retry
        return Err(err);
      }
    };

    match _updates {
      Some(updates) => {
        for update in updates {
          self.last_update = core::cmp::max(update.id, self.last_update);
          self.buffer.push_back(update);
        }
        self.updates = None;
      }
      None => {
        self.updates = None;
        try_ready!(self.error_interval.poll().map_err(|_| TGBotErrorKind::Other.into_with(|| "Interval error")));
      }
    }

    self.poll()
  }
}

//fn send<Req: TGReq>(api: &Arc<BotApi>, token: &String, req: Req) -> TGFuture<Option<<Req::Resp as TGResp>::Type>> {
////  let fut = req.request(cfg) // todo: reference cfg
////    .map(move |resp| {
////      let dez: Result<<Req::Resp as TGResp>::Type, TGBotError> = Req::Resp::deserialize(resp);
////      match dez {
////        Ok(ret) => Some(ret),
////        Err(err) => {
////          // todo: if error do more thing
////          error!(tglog::telegram(), "Call telegram api fail: {:?}", err);
////          None
////        }
////      }
////    }).map_err(|e| e);
////  TGFuture::new(Box::new(fut))
//
//}

