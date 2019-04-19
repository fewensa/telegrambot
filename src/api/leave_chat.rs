use reqwest::Method;

use crate::api::req::HttpReq;
use crate::api::resp::JsonTrueToUnitResp;
use crate::api::TGReq;
use crate::errors::TGBotResult;

/// Use this method for your bot to leave a group, supergroup or channel.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct LeaveChat {
  chat_id: i64
}


impl TGReq for LeaveChat {
  type Resp = JsonTrueToUnitResp;

  fn request(&self) -> TGBotResult<HttpReq> {
    HttpReq::json_req(Method::POST, "leaveChat", self)
  }
}

impl LeaveChat {
  pub fn new(chat: i64) -> Self  {
    LeaveChat {
      chat_id: chat
    }
  }
}
