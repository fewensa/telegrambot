use reqwest::Method;

use crate::api::req::HttpReq;
use crate::api::resp::RespType;
use crate::api::TGReq;
use crate::errors::TGBotResult;
use crate::types::*;

/// Use this method to get up to date information about the chat.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct GetChat {
  chat_id: i64
}


impl TGReq for GetChat {
  type Resp = RespType<Chat>;

  fn request(&self) -> TGBotResult<HttpReq> {
    HttpReq::json_req(Method::POST, "getChat", self)
  }
}

impl GetChat {
  pub fn new(chat: i64) -> Self {
    GetChat {
      chat_id: chat
    }
  }
}
