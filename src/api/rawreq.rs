use std::sync::Arc;

use error_chain_mini::ErrorKind;
use futures::{Future, Stream};
use reqwest::r#async::Client;
use reqwest::Url;

use crate::api;
use crate::api::req::HttpReq;
use crate::api::resp::HttpResp;
use crate::errors::TGBotErrorKind;
use crate::tgfut::TGFuture;
use crate::tglog;

#[derive(Debug, Clone)]
pub struct RawReq {
  client: Client,
  token: String,
}

impl RawReq {
  pub fn new(client: Client, token: String) -> Self {
    RawReq {
      client,
      token,
    }
  }

  pub fn request(&self, httpreq: HttpReq) -> TGFuture<HttpResp> {
    let url = Url::parse(&format!("{}/bot{}/{}", api::botapi::telegram_api_url(), self.token, httpreq.api)[..]).unwrap();
    let client = self.client.clone();

    let fut = match httpreq.body {
      Some(body) => {
        debug!(tglog::telegram(), "REQUEST URL => [{}]   REQUEST BODY => {}",
               format!("{}/bot___/{}", api::botapi::telegram_api_url(), httpreq.api),
               body);
        client.request(httpreq.method, url)
          .header("content-type", "application/json")
          .body(body)
          .send()
      }
      None => {
        debug!(tglog::telegram(), "REQUEST URL => [{}]",
               format!("{}/bot___/{}", api::botapi::telegram_api_url(), httpreq.api));
        client.request(httpreq.method, url).send()
      }
    };

    let fut = fut.and_then(|res| {
      let body = res.into_body();
      body.concat2()
    })
      .and_then(|buf| {
        if cfg!(debug_assertions) {
          match ::std::str::from_utf8(&buf) {
            Ok(body) => debug!(tglog::telegram(), "RESPONSE BODY: {:?}", body),
            Err(err) => error!(tglog::telegram(), "RESPONSE ERROR: {:?}", err)
          }
        }
        Ok(HttpResp {
          body: Some(Vec::from(buf.as_ref()))
        })
      })
      .map_err(|err| TGBotErrorKind::RequestError(err).into_err());
    TGFuture::new(Box::new(fut))
  }
}
