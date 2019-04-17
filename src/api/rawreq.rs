use std::rc::Rc;

use error_chain_mini::ErrorKind;
use futures::future::Future;
use futures::stream::Stream;
use reqwest::r#async::Client;
use reqwest::Url;

use crate::api::req::HttpReq;
use crate::api::resp::HttpResp;
use crate::api::TELEGRAM_API_URL;
use crate::errors::{TGBotErrorKind, TGBotError};
use crate::tgfut::TGFuture;
use crate::tglog;
use std::sync::Arc;

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

  pub fn request(&self, httpreq: HttpReq) -> impl Future<Item=HttpResp, Error=TGBotError> {
    let url = Url::parse(&format!("{}bot{}/{}", TELEGRAM_API_URL, self.token, httpreq.api)[..]).unwrap();
    let client = self.client.clone();
    let request = client.request(httpreq.method, url);
    if let Some(body) = httpreq.body {
      request.header("content-type", "application/json");
      request.body(body);
    }
    let fut = request.send()
      .and_then(|res| {
        let body = res.into_body();
        body.concat2()
      })
      .and_then(|buf| {
//      Ok(Some(Vec::from(buf.as_ref())))
        if cfg!(debug_assertions) {
          match ::std::str::from_utf8(&buf) {
            Ok(body) => debug!(tglog::telegram(), "RESPONSE BODY: {}", ::std::str::from_utf8(&buf).unwrap()),
            Err(err) => error!(tglog::telegram(), "RESPONSE ERROR: {:?}", err)
          }
        }
        Ok(HttpResp {
          body: Some(Vec::from(buf.as_ref()))
        })
      })
      .map_err(|err| TGBotErrorKind::RequestError(err).into_err());
//    TGFuture::new(Box::new(fut))
    fut
  }
}
