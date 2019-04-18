use reqwest::Method;

use crate::api::req::HttpReq;
use crate::api::resp::{JsonTrueToUnitResp, RespType};
use crate::api::TGReq;
use crate::errors::TGBotResult;
use crate::types::*;

/// Use this method for your bot to leave a group, supergroup or channel.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct LeaveChat {
  chat_id: ChatRef
}


impl TGReq for LeaveChat {
  type Resp = JsonTrueToUnitResp;

  fn request(&self) -> TGBotResult<HttpReq> {
    HttpReq::json_req(Method::POST, "leaveChat", self)
  }
}

impl LeaveChat {
  pub fn new<C>(chat: C) -> Self where C: ToChatRef {
    LeaveChat {
      chat_id: chat.to_chat_ref()
    }
  }
}

/// Leave a group, supergroup or channel.
pub trait CanLeaveChat {
  fn leave(&self) -> LeaveChat;
}

impl<C> CanLeaveChat for C where C: ToChatRef {
  fn leave(&self) -> LeaveChat {
    LeaveChat::new(self)
  }
}
