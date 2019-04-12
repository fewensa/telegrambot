use std::time::Duration;

use error_chain_mini::ErrorKind;
use futures::{Async, Stream};
use futures::future::Future;
use tokio::timer::Interval;

use crate::{TGBotError, TGBotErrorKind, TGFuture};
use crate::boreq::{TGReq, UpdateReq};
use crate::config::Config;
use crate::types::Update;

pub struct UpdatesStream<'cfg> {
  cfg: &'cfg Config,
  interval: Interval,
  boreq: Option<TGFuture<Option<Vec<Update>>>>,
  updreq: Option<TGFuture<Option<String>>>,
}

impl UpdatesStream {
  pub fn new(cfg: &Config) -> Self {
    UpdatesStream {
      cfg,
      interval: Interval::new_interval(Duration::from_secs(1)),
      boreq: None,
      updreq: None,
    }
  }
}

impl Stream for UpdatesStream {
  type Item = String;
  type Error = TGBotError;

  fn poll(&mut self) -> Result<Async<Option<Self::Item>>, Self::Error> {
    try_ready!(self.interval.poll().map_err(|_| TGBotErrorKind::Other.into_with(|| "Some err")));

    let upfut = self.updreq.get_or_insert_with(|| {
      self::send(UpdateReq {})
    });

    match try_ready!(upfut.poll()) {
      Some(value) => { println!("{:?}", value) }
      None => {}
    }
    self.updreq = None;

    Ok(Async::Ready(Some("abcd".to_string())))
  }
}

// -> TGFuture<REQ>
fn send<REQ: TGReq>(req: REQ) -> TGFuture<Option<String>> {
  req.request()
}

