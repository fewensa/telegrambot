use std::borrow::Cow;
use std::ops::Not;

use reqwest::Method;

use crate::api::req::HttpReq;
use crate::api::resp::JsonTrueToUnitResp;
use crate::api::TGReq;
use crate::errors::TGBotResult;
use crate::types::*;

/// Use this method to send an audio
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct SendAudio<'s, 'c, 'p, 't> {
  chat_id: i64,
  audio: Cow<'s, str>,
  #[serde(skip_serializing_if = "Option::is_none")]
  caption: Option<Cow<'c, str>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  parse_mode: Option<ParseMode>,
  #[serde(skip_serializing_if = "Option::is_none")]
  duration: Option<i64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  performer: Option<Cow<'p, str>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  title: Option<Cow<'t, str>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  reply_to_message_id: Option<i64>,
  #[serde(skip_serializing_if = "Not::not")]
  disable_notification: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  reply_markup: Option<ReplyMarkup>,
}


impl<'s, 'c, 'p, 't> TGReq for SendAudio<'s, 'c, 'p, 't> {
  type Resp = JsonTrueToUnitResp;

  fn request(&self) -> TGBotResult<HttpReq> {
    HttpReq::json_req(Method::POST, "sendAudio", self)
  }
}


impl<'s, 'c, 'p, 't> SendAudio<'s, 'c, 'p, 't> {
  pub fn with_url<T>(chat: i64, url: T) -> Self
    where
      T: Into<Cow<'s, str>>,
  {
    Self {
      chat_id: chat,
      audio: url.into(),
      caption: None,
      parse_mode: None,
      duration: None,
      performer: None,
      title: None,
      reply_to_message_id: None,
      reply_markup: None,
      disable_notification: false,
    }
  }

  pub fn caption<T>(&mut self, caption: T) -> &mut Self
    where
      T: Into<Cow<'c, str>>,
  {
    self.caption = Some(caption.into());
    self
  }

  pub fn parse_mode(&mut self, parse_mode: ParseMode) -> &mut Self {
    self.parse_mode = Some(parse_mode);
    self
  }

  pub fn duration(&mut self, duration: i64) -> &mut Self {
    self.duration = Some(duration);
    self
  }

  pub fn performer<T>(&mut self, performer: T) -> &mut Self
    where
      T: Into<Cow<'p, str>>,
  {
    self.performer = Some(performer.into());
    self
  }

  pub fn title<T>(&mut self, title: T) -> &mut Self
    where
      T: Into<Cow<'t, str>>,
  {
    self.title = Some(title.into());
    self
  }

  pub fn reply_to(&mut self, to: i64) -> &mut Self {
    self.reply_to_message_id = Some(to);
    self
  }

  pub fn reply_markup<R>(&mut self, reply_markup: R) -> &mut Self
    where
      R: Into<ReplyMarkup>,
  {
    self.reply_markup = Some(reply_markup.into());
    self
  }
}
