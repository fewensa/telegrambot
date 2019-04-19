use reqwest::Method;

use crate::api::req::HttpReq;
use crate::api::resp::RespType;
use crate::api::TGReq;
use crate::errors::TGBotResult;
use crate::types::*;

/// Use this method to get the number of members in a chat.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct GetChatMembersCount {
  chat_id: i64
}


impl TGReq for GetChatMembersCount {
  type Resp = RespType<i64>;

  fn request(&self) -> TGBotResult<HttpReq> {
    HttpReq::json_req(Method::POST, "getChatMembersCount", self)
  }
}

impl GetChatMembersCount {
  pub fn new(chat: i64) -> Self {
    GetChatMembersCount {
      chat_id: chat
    }
  }
}
