use std::ops::Not;

use reqwest::Method;

use crate::api::req::{HttpReq, ToReplyRequest, ToRequest};
use crate::api::resp::RespType;
use crate::api::TGReq;
use crate::errors::TGBotResult;
use crate::types::*;
use crate::vision::PossibilityMessage;

/// Use this method to send point on the map.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct SendLocation {
  chat_id: ChatRef,
  latitude: f32,
  longitude: f32,
  #[serde(skip_serializing_if = "Option::is_none")]
  live_period: Option<i32>,
  #[serde(skip_serializing_if = "Not::not")]
  disable_notification: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  reply_to_message_id: Option<MessageId>,
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
  pub fn new<C>(chat: C, latitude: f32, longitude: f32) -> Self where C: ToChatRef {
    SendLocation {
      chat_id: chat.to_chat_ref(),
      latitude: latitude,
      longitude: longitude,
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

  pub fn reply_to<R>(&mut self, to: R) -> &mut Self where R: ToMessageId {
    self.reply_to_message_id = Some(to.to_message_id());
    self
  }

  pub fn reply_markup<R>(&mut self, reply_markup: R) -> &mut Self where R: Into<ReplyMarkup> {
    self.reply_markup = Some(reply_markup.into());
    self
  }
}

/// Send point on the map.
pub trait CanSendLocation {
  fn location(&self, latitude: f32, longitude: f32) -> SendLocation;
}

impl<C> CanSendLocation for C where C: ToChatRef {
  fn location(&self, latitude: f32, longitude: f32) -> SendLocation {
    SendLocation::new(self, latitude, longitude)
  }
}

/// Reply with point on the map.
pub trait CanReplySendLocation {
  fn location_reply(&self, latitude: f32, longitude: f32) -> SendLocation;
}

impl<M> CanReplySendLocation for M where M: ToMessageId + ToSourceChat {
  fn location_reply(&self, latitude: f32, longitude: f32) -> SendLocation {
    let mut rq = self.to_source_chat().location(latitude, longitude);
    rq.reply_to(self.to_message_id());
    rq
  }
}

impl<'b> ToRequest<'b> for Location {
  type Request = SendLocation;

  fn to_request<C>(&'b self, chat: C) -> Self::Request where C: ToChatRef {
    chat.location(self.latitude, self.longitude)
  }
}

impl<'b> ToReplyRequest<'b> for Location {
  type Request = SendLocation;

  fn to_reply_request<M>(&'b self, message: M) -> Self::Request
    where M: ToMessageId + ToSourceChat {
    message.location_reply(self.latitude, self.longitude)
  }
}