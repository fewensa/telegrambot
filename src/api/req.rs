use std::sync::Arc;

use reqwest::Method;

use crate::api::resp::{HttpResp, TGResp};
use crate::config::Config;
use crate::errors::{TGBotErrorKind, TGBotResult};
use crate::tgfut::TGFuture;
use error_chain_mini::ErrorKind;
use serde::Serialize;

pub trait TGReq {
  type Resp: TGResp + 'static;

  fn request(&self) -> TGBotResult<HttpReq>;
}

impl<'a, Req: TGReq> TGReq for &'a Req {
  type Resp = Req::Resp;

  fn request(&self) -> TGBotResult<HttpReq> {
    (*self).request()
  }
}

impl<'a, Req: TGReq> TGReq for &'a mut Req {
  type Resp = Req::Resp;

  fn request(&self) -> TGBotResult<HttpReq> {
    (**self).request()
  }
}

//pub enum HttpMethod {
//  Get,
//  Post,
//}

pub struct HttpReq {
  pub api: &'static str,
  pub method: Method,
  pub body: Option<String>,
}

impl HttpReq {
  pub fn no_body_req(method: Method, api: &'static str) -> TGBotResult<Self> {
    Ok(HttpReq {
      api,
      method,
      body: None,
    })
  }

  pub fn json_req<Req>(method: Method,
                       api: &'static str,
                       body: &Req) -> TGBotResult<Self> where Req: TGReq + Serialize {
    let json = serde_json::to_string(body)
      .map_err(|e| TGBotErrorKind::JsonError(e).into_err())?;
    Ok(HttpReq {
      api,
      method,
      body: Some(json),
    })
  }
}
