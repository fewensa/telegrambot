use std::ops::Not;

use reqwest::Method;

use crate::api::req::HttpReq;
use crate::api::resp::RespType;
use crate::api::TGReq;
use crate::errors::TGBotResult;
use crate::types::*;
use crate::vision::PossibilityMessage;

/// Use this method to send point on the map.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct SendLocation {
  chat_id: i64,
  latitude: f32,
  longitude: f32,
  #[serde(skip_serializing_if = "Option::is_none")]
  live_period: Option<i32>,
  #[serde(skip_serializing_if = "Not::not")]
  disable_notification: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  reply_to_message_id: Option<i64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  reply_markup: Option<ReplyMarkup>,
}

impl TGReq for SendLocation {
  type Resp = RespType<PossibilityMessage>;

  fn request(&self) -> TGBotResult<HttpReq> {
    HttpReq::json_req(Method::POST, "sendLocation", self)
  }
}


impl SendLocation {
  pub fn new(chat: i64, latitude: f32, longitude: f32) -> Self {
    SendLocation {
      chat_id: chat,
      latitude,
      longitude,
      live_period: None,
      disable_notification: false,
      reply_to_message_id: None,
      reply_markup: None,
    }
  }

  /// Period in seconds for which the location will be updated, should be between 60 and 86400.
  pub fn live_period(&mut self, period: i32) -> &mut Self {
    self.live_period = Some(period);
    self
  }

  pub fn disable_notification(&mut self) -> &mut Self {
    self.disable_notification = true;
    self
  }

  pub fn reply_to(&mut self, to: i64) -> &mut Self {
    self.reply_to_message_id = Some(to);
    self
  }

  pub fn reply_markup<R>(&mut self, reply_markup: R) -> &mut Self where R: Into<ReplyMarkup> {
    self.reply_markup = Some(reply_markup.into());
    self
  }
}
