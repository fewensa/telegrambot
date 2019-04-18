use error_chain_mini::ChainedError;
use reqwest::Method;

use crate::api::req::HttpReq;
use crate::api::resp::RespType;
use crate::api::TGReq;
use crate::errors::TGBotErrorKind;
use crate::types::{ChatRef, MessageId, ReplyMarkup, ToChatRef, ToMessageId, ToSourceChat};
use crate::vision::PossibilityMessage;

/// Use this method to edit live location messages sent by the bot.
/// A location can be edited until its live_period expires or editing
/// is explicitly disabled by a call to stopMessageLiveLocation.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct EditMessageLiveLocation {
  chat_id: ChatRef,
  message_id: MessageId,
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
  pub fn new<C, M>(chat: C, message_id: M, latitude: f32, longitude: f32) -> Self
    where C: ToChatRef, M: ToMessageId {
    EditMessageLiveLocation {
      chat_id: chat.to_chat_ref(),
      message_id: message_id.to_message_id(),
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

/// Edit live location messages sent by the bot.
pub trait CanEditMessageLiveLocation {
  fn edit_live_location(&self, latitude: f32, longitude: f32) -> EditMessageLiveLocation;
}

impl<M> CanEditMessageLiveLocation for M where M: ToMessageId + ToSourceChat {
  fn edit_live_location(&self, latitude: f32, longitude: f32) -> EditMessageLiveLocation {
    EditMessageLiveLocation::new(self.to_source_chat(), self.to_message_id(), latitude, longitude)
  }
}
