use futures::{Future, Poll};

use crate::errors::TGBotError;

/// Represent a future that resolves into Telegram API response.
#[must_use = "futures do nothing unless polled"]
pub struct TGFuture<T> {
  inner: Box<Future<Item=T, Error=TGBotError> + Send>
}


impl<T> TGFuture<T> {
  pub fn new(inner: Box<Future<Item=T, Error=TGBotError> + Send>) -> Self {
    Self { inner }
  }
}

impl<T> Future for TGFuture<T> {
  type Item = T;
  type Error = TGBotError;

  fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
    self.inner.poll()
  }
}
