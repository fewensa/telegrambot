use std::sync::Arc;

use reqwest::Method;

use crate::api::{TGReq};
use crate::api::req::HttpReq;
use crate::api::resp::{HttpResp, RespType};
use crate::config::Config;
use crate::errors::TGBotResult;
use crate::tgfut::TGFuture;
use crate::types::Update;

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
pub struct GetUpdates {
  #[serde(skip_serializing_if = "Option::is_none")]
  offset: Option<i64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  limit: Option<i64>,
  // TODO(knsd): Values between 1—100 are accepted
  #[serde(skip_serializing_if = "Option::is_none")]
  timeout: Option<i64>,
  // TODO(knsd): Should be positive
  allowed_updates: Vec<AllowedUpdate>, // TODO(knsd) BitSet? HashSet? BTreeSet?
}

impl GetUpdates {
  pub fn new() -> Self {
    GetUpdates {
      offset: None,
      limit: None,
      timeout: None,
      allowed_updates: Vec::new(),
    }
  }

  pub fn offset(&mut self, offset: i64) -> &mut Self {
    self.offset = Some(offset);
    self
  }

  pub fn limit(&mut self, limit: i64) -> &mut Self {
    self.limit = Some(limit);
    self
  }

  pub fn timeout(&mut self, timeout: i64) -> &mut Self {
    self.timeout = Some(timeout);
    self
  }

  pub fn allowed_updates(&mut self, updates: &[AllowedUpdate]) -> &mut Self {
    self.allowed_updates = updates.to_vec();
    self
  }
}

impl TGReq for GetUpdates {
  type Resp = RespType<Vec<Update>>;

  fn request(&self) -> TGBotResult<HttpReq> {
    HttpReq::json_req(Method::POST, "getUpdates", self)
  }
}


#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
pub enum AllowedUpdate {
  #[serde(rename = "message")]
  Message,
  #[serde(rename = "edited_message")]
  EditedMessage,
  #[serde(rename = "channel_post")]
  ChannelPost,
  #[serde(rename = "edited_channel_post")]
  EditedChannelPost,
//    #[serde(rename="inline_query")]
//    InlineQuery,
//    #[serde(rename="chosen_inline_query")]
//    ChosenInlineResult,
//    #[serde(rename="callback_query")]
//    CallbackQuery,
}


