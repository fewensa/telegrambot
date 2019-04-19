use std::ops::Not;

use reqwest::Method;

use crate::api::req::HttpReq;
use crate::api::resp::RespType;
use crate::api::TGReq;
use crate::errors::TGBotResult;
use crate::types::*;
use crate::vision::PossibilityMessage;

/// Use this method to forward messages of any kind.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct ForwardMessage {
  chat_id: i64,
  from_chat_id: i64,
  #[serde(skip_serializing_if = "Not::not")]
  disable_notification: bool,
  message_id: i64,
}

impl TGReq for ForwardMessage {
  type Resp = RespType<PossibilityMessage>;

  fn request(&self) -> TGBotResult<HttpReq> {
    HttpReq::json_req(Method::POST, "editMessageCaption", self)
  }
}

impl ForwardMessage {
  pub fn new(message: i64, from: i64, to: i64) -> Self  {
    ForwardMessage {
      chat_id: to,
      from_chat_id: from,
      disable_notification: false,
      message_id: message,
    }
  }

  pub fn disable_notification(&mut self) -> &mut Self {
    self.disable_notification = true;
    self
  }
}
