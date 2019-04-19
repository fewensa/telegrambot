use std::ops::Not;

use reqwest::Method;

use crate::api::req::HttpReq;
use crate::api::resp::{JsonTrueToUnitResp, RespType};
use crate::api::TGReq;
use crate::errors::TGBotResult;
use crate::types::*;

/// Use this method to pin a message in a supergroup or a channel.
/// The bot must be an administrator in the chat for this to work
/// and must have the ‘can_pin_messages’ admin right in the supergroup
/// or ‘can_edit_messages’ admin right in the channel.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct PinChatMessage {
  chat_id: i64,
  message_id: i64,
  #[serde(skip_serializing_if = "Not::not")]
  disable_notification: bool,
}


impl TGReq for PinChatMessage {
  type Resp = JsonTrueToUnitResp;

  fn request(&self) -> TGBotResult<HttpReq> {
    HttpReq::json_req(Method::POST, "pinChatMessage", self)
  }
}


impl PinChatMessage {
  fn new(chat: i64, message: i64) -> Self{
    Self {
      chat_id: chat,
      message_id: message,
      disable_notification: false,
    }
  }

  pub fn disable_notification(&mut self) -> &mut Self {
    self.disable_notification = true;
    self
  }
}
