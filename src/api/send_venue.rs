use std::borrow::Cow;
use std::ops::Not;

use reqwest::Method;

use crate::api::req::HttpReq;
use crate::api::resp::RespType;
use crate::api::TGReq;
use crate::errors::TGBotResult;
use crate::types::*;
use crate::vision::PossibilityMessage;

/// Use this method to send information about a venue.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct SendVenue<'t, 'a, 'f> {
  chat_id: i64,
  latitude: f32,
  longitude: f32,
  title: Cow<'t, str>,
  address: Cow<'a, str>,
  #[serde(skip_serializing_if = "Option::is_none")]
  foursquare_id: Option<Cow<'f, str>>,
  #[serde(skip_serializing_if = "Not::not")]
  disable_notification: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  reply_to_message_id: Option<i64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  reply_markup: Option<ReplyMarkup>,
}


impl<'t, 'a, 'f> TGReq for SendVenue<'t, 'a, 'f> {
  type Resp = RespType<PossibilityMessage>;

  fn request(&self) -> TGBotResult<HttpReq> {
    HttpReq::json_req(Method::POST, "sendVenue", self)
  }
}

impl<'t, 'a, 'f> SendVenue<'t, 'a, 'f> {
  pub fn new<T, A>(chat: i64, latitude: f32, longitude: f32, title: T, address: A) -> Self
    where T: Into<Cow<'t, str>>,
          A: Into<Cow<'a, str>>
  {
    SendVenue {
      chat_id: chat,
      latitude,
      longitude,
      title: title.into(),
      address: address.into(),
      disable_notification: false,
      foursquare_id: None,
      reply_to_message_id: None,
      reply_markup: None,
    }
  }

  pub fn disable_notification(&mut self) -> &mut Self {
    self.disable_notification = true;
    self
  }

  pub fn foursquare_id<F>(&mut self, id: F) -> &mut Self
    where F: Into<Cow<'f, str>>
  {
    self.foursquare_id = Some(id.into());
    self
  }

  pub fn reply_to(&mut self, to: i64) -> &mut Self {
    self.reply_to_message_id = Some(to);
    self
  }

  pub fn reply_markup<R>(&mut self, reply_markup: R) -> &mut Self
    where R: Into<ReplyMarkup>
  {
    self.reply_markup = Some(reply_markup.into());
    self
  }
}
