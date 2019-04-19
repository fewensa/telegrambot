use std::borrow::Cow;
use std::ops::Not;

use reqwest::Method;

use crate::api::req::HttpReq;
use crate::api::resp::RespType;
use crate::api::TGReq;
use crate::errors::{TGBotResult, TGBotErrorKind};
use crate::types::*;
use crate::vision::PossibilityMessage;
use error_chain_mini::ErrorKind;

/// Use this method to send text messages.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct SendMessage<'s> {
  chat_id: i64,
  text: Cow<'s, str>,
  #[serde(skip_serializing_if = "Option::is_none")]
  parse_mode: Option<ParseMode>,
  #[serde(skip_serializing_if = "Not::not")]
  disable_web_page_preview: bool,
  #[serde(skip_serializing_if = "Not::not")]
  disable_notification: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  reply_to_message_id: Option<i64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  reply_markup: Option<ReplyMarkup>,
}


impl<'c, 's> TGReq for SendMessage<'s> {
  type Resp = RespType<PossibilityMessage>;

  fn request(&self) -> TGBotResult<HttpReq> {
    HttpReq::json_req(Method::POST, "sendMessage", self)
  }
}


impl<'s> SendMessage<'s> {
  pub fn new<T>(chat: i64, text: T) -> Self where T: Into<Cow<'s, str>> {
    SendMessage {
      chat_id: chat,
      text: text.into(),
      parse_mode: None,
      disable_web_page_preview: false,
      disable_notification: false,
      reply_to_message_id: None,
      reply_markup: None,
    }
  }

  pub fn parse_mode(&mut self, parse_mode: ParseMode) -> &mut Self {
    self.parse_mode = Some(parse_mode);
    self
  }

  pub fn disable_preview(&mut self) -> &mut Self {
    self.disable_web_page_preview = true;
    self
  }

  pub fn disable_notification(&mut self) -> &mut Self {
    self.disable_notification = true;
    self
  }

  pub fn reply_to(&mut self, to: i64) -> &mut Self{
    self.reply_to_message_id = Some(to);
    self
  }

  pub fn reply_markup<R>(&mut self, reply_markup: R) -> &mut Self where R: Into<ReplyMarkup> {
    self.reply_markup = Some(reply_markup.into());
    self
  }
}
