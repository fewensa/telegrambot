use reqwest::Method;

use crate::api::req::HttpReq;
use crate::api::resp::{RespType, JsonTrueToUnitResp};
use crate::api::TGReq;
use crate::errors::TGBotResult;

/// Use this method to kick a user from a group or a supergroup.
/// In the case of supergroups, the user will not be able to return to the group on
/// their own using invite links, etc., unless unbanned first.
/// The bot must be an administrator in the group for this to work.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct KickChatMember {
  chat_id: i64,
  user_id: i64,
}


impl TGReq for KickChatMember {
  type Resp = JsonTrueToUnitResp;

  fn request(&self) -> TGBotResult<HttpReq> {
    HttpReq::json_req(Method::POST, "kickChatMember", self)
  }
}

impl KickChatMember {
  pub fn new(chat: i64, user: i64) -> Self {
    KickChatMember {
      chat_id: chat,
      user_id: user,
    }
  }
}
