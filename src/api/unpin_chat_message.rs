use reqwest::Method;

use crate::api::req::HttpReq;
use crate::api::resp::JsonTrueToUnitResp;
use crate::api::TGReq;
use crate::errors::TGBotResult;

///Use this method to unpin a message in a supergroup or a channel.
/// The bot must be an administrator in the chat for this to work
/// and must have the ‘can_pin_messages’ admin right in the
/// supergroup or ‘can_edit_messages’ admin right in the channel.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct UnpinChatMessage {
  chat_id: i64,
}

impl TGReq for UnpinChatMessage {
  type Resp = JsonTrueToUnitResp;

  fn request(&self) -> TGBotResult<HttpReq> {
    HttpReq::json_req(Method::POST, "unpinChatMessage", self)
  }
}

impl UnpinChatMessage {
  pub fn new(chat: i64) -> Self {
    Self {
      chat_id: chat,
    }
  }
}
