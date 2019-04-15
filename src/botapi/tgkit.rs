use std::sync::Arc;

use error_chain_mini::ErrorKind;
use futures::future::Future;
use futures::stream::Stream;
use reqwest::Url;
use serde::Serialize;

use crate::botapi::resp::HttpResp;
use crate::botapi::TGReq;
use crate::config::Config;
use crate::errors::TGBotErrorKind;
use crate::tgfut::TGFuture;
use crate::tglog;

pub const TELEGRAM_API_URL: &'static str = "https://tgb.akafwtll.tk/";

//lazy_static! {
//  /// Documentation!
//  pub static ref TELEGRAM_API_URL: &'static str = tgapiurl();
//}

//pub static TELEGRAM_API_URL: &'static str = tgapiurl();


fn api<S>(token: String, method: S) -> Url where S: AsRef<str> {
  Url::parse(&format!("{}bot{}/{}", TELEGRAM_API_URL, token, method.as_ref().to_string())[..]).unwrap()
}

fn telegram_api_url<'a>() -> &'a str {
  if cfg!(debug_assertions) {
    "https://tgb.akafwtll.tk/"
  } else {
    "https://api.telegram.org/"
  }
}

pub fn exec<Req, S>(cfg: Arc<Config>, method: S, req: &Req)
                    -> TGFuture<HttpResp>
  where Req: TGReq + Serialize, S: AsRef<str> {
  let method_api = method.as_ref().to_string();
  let api = Url::parse(&format!("{}bot{}/{}", TELEGRAM_API_URL, cfg.token(), method_api)[..]).unwrap();
  let client = cfg.client();

  let json = serde_json::to_string(req).unwrap();
  debug!(tglog::telegram(), "REQUEST URL => [{}]   REQUEST BODY => {}",
         format!("{}bot___/{}", TELEGRAM_API_URL, method_api),
         json);

  let reqfut = client.post(api)
    .header("content-type", "application/json")
    .body(json)
    .send()
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

  TGFuture::new(Box::new(reqfut))
}

