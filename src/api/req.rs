use error_chain_mini::ErrorKind;
use reqwest::Method;
use serde::Serialize;

use crate::api::resp::TGResp;
use crate::errors::{TGBotErrorKind, TGBotResult};
use crate::types::{ToChatRef, ToMessageId, ToSourceChat};

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


/// Use this trait to convert a complex type to corresponding request and send it to the chat.
pub trait ToRequest<'b> {
  /// Request type.
  type Request: TGReq;

  /// Convert type to request and send it to the chat.
  fn to_request<C>(&'b self, chat: C) -> Self::Request where C: ToChatRef;
}

/// Use this trait to convert a complex type to corresponding request and reply to the message.
pub trait ToReplyRequest<'b> {
  /// Request type.
  type Request: TGReq;

  /// Convert type to request and reply to the message.
  fn to_reply_request<M>(&'b self, message: M) -> Self::Request
    where M: ToMessageId + ToSourceChat;
}
