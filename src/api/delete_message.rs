use reqwest::Method;

use crate::api::req::HttpReq;
use crate::api::resp::JsonTrueToUnitResp;
use crate::api::TGReq;
use crate::errors::TGBotResult;

// Use this method to delete a message.
// A message can only be deleted if it was sent less than 48 hours ago.
// Any such recently sent outgoing message may be deleted.
// Additionally, if the bot is an administrator in a group chat, it can delete any message.
// If the bot is an administrator in a supergroup, it can delete messages from any
// other user and service messages about people joining or leaving the
// group (other types of service messages may only be removed by the group creator).
// In channels, bots can only remove their own messages.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct DeleteMessage {
  chat_id: i64,
  message_id: i64,
}

impl TGReq for DeleteMessage {
  type Resp = JsonTrueToUnitResp;

  fn request(&self) -> TGBotResult<HttpReq> {
    HttpReq::json_req(Method::POST, "deleteMessage", self)
  }
}

impl DeleteMessage {
  pub fn new(chat: i64, message_id: i64) -> Self {
    DeleteMessage {
      chat_id: chat,
      message_id,
    }
  }
}
