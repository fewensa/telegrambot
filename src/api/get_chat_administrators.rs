use reqwest::Method;

use crate::api::req::HttpReq;
use crate::api::resp::RespType;
use crate::api::TGReq;
use crate::errors::TGBotResult;
use crate::types::*;

/// Use this method to get a list of administrators in a chat.
/// If the chat is a group or a supergroup and no administrators were appointed,
/// only the creator will be returned.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct GetChatAdministrators {
  chat_id: i64
}


impl TGReq for GetChatAdministrators {
  type Resp = RespType<Vec<ChatMember>>;

  fn request(&self) -> TGBotResult<HttpReq> {
    HttpReq::json_req(Method::POST, "getChatAdministrators", self)
  }
}

impl GetChatAdministrators {
  pub fn new(chat: i64) -> Self {
    GetChatAdministrators {
      chat_id: chat
    }
  }
}
