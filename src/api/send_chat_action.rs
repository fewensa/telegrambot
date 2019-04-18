use reqwest::Method;

use crate::api::req::HttpReq;
use crate::api::resp::{JsonTrueToUnitResp, RespType};
use crate::api::TGReq;
use crate::errors::TGBotResult;
use crate::types::*;

/// Strongly typed ChatAction. Instead of passing a String to the
/// `chat_action` method, this is used.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub enum ChatAction {
  #[serde(rename = "typing")]
  Typing,
  #[serde(rename = "upload_photo")]
  UploadPhoto,
  #[serde(rename = "record_video")]
  RecordVideo,
  #[serde(rename = "upload_video")]
  UploadVideo,
  #[serde(rename = "record_audio")]
  RecordAudio,
  #[serde(rename = "upload_audio")]
  UploadAudio,
  #[serde(rename = "upload_document")]
  UploadDocument,
  #[serde(rename = "find_location")]
  FindLocation,
}

/// Use this method when you need to tell the user that something is happening on the bot's side.
/// The status is set for 5 seconds or less (when a message arrives from your bot,
/// Telegram clients clear its typing status).
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
pub struct SendChatAction {
  chat_id: ChatRef,
  action: ChatAction,
}

impl TGReq for SendChatAction {
  type Resp = JsonTrueToUnitResp;

  fn request(&self) -> TGBotResult<HttpReq> {
    HttpReq::json_req(Method::POST, "sendChatAction", self)
  }
}

impl SendChatAction {
  pub fn new<C>(chat: C, action: ChatAction) -> Self where C: ToChatRef {
    SendChatAction {
      chat_id: chat.to_chat_ref(),
      action: action,
    }
  }
}

/// Send `action` to a chat.
pub trait CanSendChatAction {
  fn chat_action(&self, action: ChatAction) -> SendChatAction;
}

impl<C> CanSendChatAction for C where C: ToChatRef {
  fn chat_action(&self, action: ChatAction) -> SendChatAction {
    SendChatAction::new(self, action)
  }
}
