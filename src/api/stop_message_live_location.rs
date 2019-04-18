use reqwest::Method;

use crate::api::req::HttpReq;
use crate::api::resp::RespType;
use crate::api::TGReq;
use crate::errors::TGBotResult;
use crate::types::*;
use crate::vision::PossibilityMessage;

/// Use this method to stop updating a live location message sent by the bot
/// before live_period expires.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct StopMessageLiveLocation {
  chat_id: ChatRef,
  message_id: MessageId,
  #[serde(skip_serializing_if = "Option::is_none")]
  reply_markup: Option<ReplyMarkup>,
}


impl TGReq for StopMessageLiveLocation {
  type Resp = RespType<PossibilityMessage>;

  fn request(&self) -> TGBotResult<HttpReq> {
    HttpReq::json_req(Method::POST, "stopMessageLiveLocation", self)
  }
}

impl StopMessageLiveLocation {
  pub fn new<C, M>(chat: C, message_id: M) -> Self
    where C: ToChatRef, M: ToMessageId {
    StopMessageLiveLocation {
      chat_id: chat.to_chat_ref(),
      message_id: message_id.to_message_id(),
      reply_markup: None,
    }
  }

  pub fn reply_markup<R>(&mut self, reply_markup: R) -> &mut Self where R: Into<ReplyMarkup> {
    self.reply_markup = Some(reply_markup.into());
    self
  }
}

/// Stop updating a live location message sent by the bot.
pub trait CanStopMessageLiveLocation {
  fn stop_live_location(&self) -> StopMessageLiveLocation;
}

impl<M> CanStopMessageLiveLocation for M where M: ToMessageId + ToSourceChat {
  fn stop_live_location(&self) -> StopMessageLiveLocation {
    StopMessageLiveLocation::new(self.to_source_chat(), self.to_message_id())
  }
}
