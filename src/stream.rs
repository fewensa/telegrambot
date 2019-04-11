
use futures::{Stream, Async};
use crate::{TGBotError, TGBotErrorKind, TGFuture};
use error_chain_mini::ErrorKind;
use tokio::timer::Interval;
use std::time::Duration;
use crate::types::Update;
use crate::boreq::TGReq;

pub struct UpdatesStream {
  interval: Interval,
  boreq: Option<TGFuture<Option<Vec<Update>>>>
}

impl UpdatesStream {
  pub fn new() -> Self {
    UpdatesStream {
      interval: Interval::new_interval(Duration::from_secs(1)),
      boreq: None
    }
  }
}

impl Stream for UpdatesStream {
  type Item = String;
  type Error = TGBotError;

  fn poll(&mut self) -> Result<Async<Option<Self::Item>>, Self::Error> {
    try_ready!(self.interval.poll().map_err(|_| TGBotErrorKind::Other.into_with(|| "Some err")));

    Ok(Async::Ready(Some("abcd".to_string())))
  }
}

fn send<REQ: TGReq>(req: REQ) -> TGFuture<REQ> {

}

