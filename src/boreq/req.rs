use error_chain_mini::ErrorKind;
use futures::future::Future;
use futures::stream::{Concat2, Stream};
use reqwest::r#async::Client;

use crate::{TGBotErrorKind, TGFuture};
use crate::config::Config;
use crate::types::Update;

pub trait TGReq {
//  type Resp: TGRet + 'static;

  fn request(&self, cfg: &Config) -> TGFuture<Option<String>>;
}

pub trait TGRet {}


pub struct UpdateReq {}

impl TGReq for UpdateReq {
  fn request(&self, cfg: &Config) -> TGFuture<Option<String>> {

    let reqfut = cfg.client()?
      .get("https://hyper.rs")
      .send()
      .and_then(|res| {
        let body = res.into_body();
        body.concat2()
      })
      .and_then(|buf| {
        let body = ::std::str::from_utf8(&buf).unwrap();
        Ok(Some(body.to_string()))
      })
      .map_err(|err| TGBotErrorKind::RequestError(err).into_err());

    TGFuture::new(Box::new(reqfut))
  }
}
