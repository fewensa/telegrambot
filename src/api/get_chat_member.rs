use reqwest::Method;

use crate::api::req::HttpReq;
use crate::api::resp::RespType;
use crate::api::TGReq;
use crate::errors::TGBotResult;
use crate::types::*;

/// Use this method to get information about a member of a chat.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct GetChatMember {
  chat_id: i64,
  user_id: i64,
}


impl TGReq for GetChatMember {
  type Resp = RespType<ChatMember>;

  fn request(&self) -> TGBotResult<HttpReq> {
    HttpReq::json_req(Method::POST, "getChatMember", self)
  }
}


impl GetChatMember {
  pub fn new(chat: i64, user: i64) -> Self {
    GetChatMember {
      chat_id: chat,
      user_id: user,
    }
  }
}
