use error_chain_mini::ChainedError;
use reqwest::Method;

use crate::api::req::HttpReq;
use crate::api::resp::RespType;
use crate::api::TGReq;
use crate::errors::TGBotErrorKind;
use crate::types::ReplyMarkup;
use crate::vision::PossibilityMessage;

/// Use this method to edit live location messages sent by the bot.
/// A location can be edited until its live_period expires or editing
/// is explicitly disabled by a call to stopMessageLiveLocation.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct EditMessageLiveLocation {
  chat_id: i64,
  message_id: i64,
  latitude: f32,
  longitude: f32,
  #[serde(skip_serializing_if = "Option::is_none")]
  reply_markup: Option<ReplyMarkup>,
}

impl TGReq for EditMessageLiveLocation {
  type Resp = RespType<PossibilityMessage>;

  fn request(&self) -> Result<HttpReq, ChainedError<TGBotErrorKind>> {
    HttpReq::json_req(Method::POST, "editMessageLiveLocation", self)
  }
}

impl EditMessageLiveLocation {
  pub fn new(chat: i64, message_id: i64, latitude: f32, longitude: f32) -> Self {
    EditMessageLiveLocation {
      chat_id: chat,
      message_id,
      latitude,
      longitude,
      reply_markup: None,
    }
  }

  pub fn reply_markup<R>(&mut self, reply_markup: R) -> &mut Self where R: Into<ReplyMarkup> {
    self.reply_markup = Some(reply_markup.into());
    self
  }
}
