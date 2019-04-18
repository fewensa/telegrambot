use std::sync::Arc;

use error_chain_mini::ErrorKind;
use futures::{Future, Stream};
use reqwest::r#async::Client;
use reqwest::Url;

use crate::api::req::HttpReq;
use crate::api::resp::HttpResp;
use crate::api::TELEGRAM_API_URL;
use crate::errors::TGBotErrorKind;
use crate::tgfut::TGFuture;
use crate::tglog;

#[derive(Debug, Clone)]
pub struct RawReq {
  client: Arc<Client>,
  token: String,
}

impl RawReq {
  pub fn new(client: Arc<Client>, token: String) -> Self {
    RawReq {
      client,
      token,
    }
  }

  //  pub fn request(&self, httpreq: HttpReq) -> impl Future<Item=HttpResp, Error=TGBotError> {
  pub fn request(&self, httpreq: HttpReq) -> TGFuture<HttpResp> {
    let url = Url::parse(&format!("{}bot{}/{}", TELEGRAM_API_URL, self.token, httpreq.api)[..]).unwrap();
    let client = self.client.clone();

    let fut = match httpreq.body {
      Some(body) => {
        debug!(tglog::telegram(), "REQUEST URL => [{}]   REQUEST BODY => {}",
               format!("{}bot___/{}", TELEGRAM_API_URL, httpreq.api),
               body);
        client.request(httpreq.method, url)
          .header("content-type", "application/json")
          .body(body)
          .send()
      }
      None => {
        debug!(tglog::telegram(), "REQUEST URL => [{}]",
               format!("{}bot___/{}", TELEGRAM_API_URL, httpreq.api));
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
            Ok(body) => debug!(tglog::telegram(), "RESPONSE BODY: {}", body),
            Err(err) => error!(tglog::telegram(), "RESPONSE ERROR: {:?}", err)
          }
        }
        Ok(HttpResp {
          body: Some(Vec::from(buf.as_ref()))
        })
      })
//    .map_err(|err| println!("request error: {}", err))
      .map_err(|err| TGBotErrorKind::RequestError(err).into_err());
    TGFuture::new(Box::new(fut))
  }
}
