use std::borrow::Cow;

use reqwest::Method;

use crate::api::req::HttpReq;
use crate::api::resp::RespType;
use crate::api::TGReq;
use crate::errors::TGBotResult;
use crate::types::ReplyMarkup;
use crate::vision::PossibilityMessage;

/// Use this method to edit captions of messages sent by the bot.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct EditMessageCaption<'s> {
  chat_id: i64,
  message_id: i64,
  caption: Cow<'s, str>,
  #[serde(skip_serializing_if = "Option::is_none")]
  reply_markup: Option<ReplyMarkup>,
}

impl<'s> TGReq for EditMessageCaption<'s> {
  type Resp = RespType<PossibilityMessage>;

  fn request(&self) -> TGBotResult<HttpReq> {
    HttpReq::json_req(Method::POST, "editMessageCaption", self)
  }
}

impl<'s> EditMessageCaption<'s> {
  pub fn new<T>(chat: i64, message_id: i64, caption: T) -> Self
    where T: Into<Cow<'s, str>> {
    EditMessageCaption {
      chat_id: chat,
      message_id,
      caption: caption.into(),
      reply_markup: None,
    }
  }

  pub fn reply_markup<R>(&mut self, reply_markup: R) -> &mut Self where R: Into<ReplyMarkup> {
    self.reply_markup = Some(reply_markup.into());
    self
  }
}
