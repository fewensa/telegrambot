use reqwest::Method;

use crate::api::req::HttpReq;
use crate::api::resp::{RespType, JsonTrueToUnitResp};
use crate::api::TGReq;
use crate::errors::TGBotResult;
use crate::types::*;

/// Use this method to unban a previously kicked user in a supergroup or channel.
/// The user will not return to the group or channel automatically, but will be able to
/// join via link, etc. The bot must be an administrator in the group for this to work.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct UnbanChatMember {
  chat_id: i64,
  user_id: i64,
}


impl TGReq for UnbanChatMember {
  type Resp = JsonTrueToUnitResp;

  fn request(&self) -> TGBotResult<HttpReq> {
    HttpReq::json_req(Method::POST, "unbanChatMember", self)
  }
}


impl UnbanChatMember {
  pub fn new(chat: i64, user: i64) -> Self {
    UnbanChatMember {
      chat_id: chat,
      user_id: user,
    }
  }
}
