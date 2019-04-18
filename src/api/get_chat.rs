use reqwest::Method;

use crate::api::req::HttpReq;
use crate::api::resp::RespType;
use crate::api::TGReq;
use crate::errors::TGBotResult;
use crate::types::*;

/// Use this method to get up to date information about the chat.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct GetChat {
  chat_id: ChatRef
}


impl TGReq for GetChat {
  type Resp = RespType<Chat>;

  fn request(&self) -> TGBotResult<HttpReq> {
    HttpReq::json_req(Method::POST, "getChat", self)
  }
}

impl GetChat {
  pub fn new<C>(chat: C) -> Self where C: ToChatRef {
    GetChat {
      chat_id: chat.to_chat_ref()
    }
  }
}

/// Get up to date information about the chat.
pub trait CanGetChat {
  fn get_chat(&self) -> GetChat;
}

impl<C> CanGetChat for C where C: ToChatRef {
  fn get_chat(&self) -> GetChat {
    GetChat::new(self)
  }
}
