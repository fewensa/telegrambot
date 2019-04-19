use std::borrow::Cow;
use std::ops::Not;

use reqwest::Method;

use crate::api::req::HttpReq;
use crate::api::resp::RespType;
use crate::api::TGReq;
use crate::errors::TGBotResult;
use crate::types::*;
use crate::vision::PossibilityMessage;

/// Use this method to send phone contacts.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct SendContact<'p, 'f, 'l> {
  chat_id: i64,
  phone_number: Cow<'p, str>,
  first_name: Cow<'f, str>,
  #[serde(skip_serializing_if = "Option::is_none")]
  last_name: Option<Cow<'l, str>>,
  #[serde(skip_serializing_if = "Not::not")]
  disable_notification: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  reply_to_message_id: Option<i64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  reply_markup: Option<ReplyMarkup>,
}


impl<'p, 'f, 'l> TGReq for SendContact<'p, 'f, 'l> {
  type Resp = RespType<PossibilityMessage>;

  fn request(&self) -> TGBotResult<HttpReq> {
    HttpReq::json_req(Method::POST, "sendContact", self)
  }
}


impl<'p, 'f, 'l> SendContact<'p, 'f, 'l> {
  pub fn new<P, F>(chat: i64, phone_number: P, first_name: F) -> Self
    where P: Into<Cow<'p, str>>,
          F: Into<Cow<'f, str>>
  {
    SendContact {
      chat_id: chat,
      phone_number: phone_number.into(),
      first_name: first_name.into(),
      last_name: None,
      disable_notification: false,
      reply_to_message_id: None,
      reply_markup: None,
    }
  }

  pub fn last_name<F>(&mut self, last_name: F) -> &mut Self
    where F: Into<Cow<'l, str>>
  {
    self.last_name = Some(last_name.into());
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

  pub fn reply_markup<R>(&mut self, reply_markup: R) -> &mut Self
    where R: Into<ReplyMarkup>
  {
    self.reply_markup = Some(reply_markup.into());
    self
  }
}
