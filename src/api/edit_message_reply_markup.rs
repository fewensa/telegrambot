use reqwest::Method;

use crate::api::req::HttpReq;
use crate::api::resp::RespType;
use crate::api::TGReq;
use crate::errors::TGBotResult;
use crate::types::*;
use crate::vision::PossibilityMessage;

/// Use this method to edit only the reply markup of messages sent by the bot.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct EditMessageReplyMarkup {
  chat_id: i64,
  message_id: i64,
  #[serde(skip_serializing_if = "Option::is_none")]
  reply_markup: Option<ReplyMarkup>,
}

impl TGReq for EditMessageReplyMarkup {
  type Resp = RespType<PossibilityMessage>;

  fn request(&self) -> TGBotResult<HttpReq> {
    HttpReq::json_req(Method::POST, "editMessageCaption", self)
  }
}

impl EditMessageReplyMarkup {
  pub fn new<R>(chat: i64, message_id: i64, reply_markup: Option<R>) -> Self
    where R: Into<ReplyMarkup> {
    EditMessageReplyMarkup {
      chat_id: chat,
      message_id,
      reply_markup: reply_markup.map(|r| r.into()),
    }
  }
}
