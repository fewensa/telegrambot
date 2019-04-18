use reqwest::Method;

use crate::api::req::HttpReq;
use crate::api::resp::RespType;
use crate::api::TGReq;
use crate::errors::TGBotResult;
use crate::types::*;

/// Use this method to get information about a member of a chat.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct GetChatMember {
  chat_id: ChatRef,
  user_id: UserId,
}


impl TGReq for GetChatMember {
  type Resp = RespType<ChatMember>;

  fn request(&self) -> TGBotResult<HttpReq> {
    HttpReq::json_req(Method::POST, "getChatMember", self)
  }
}


impl GetChatMember {
  pub fn new<C, U>(chat: C, user: U) -> Self where C: ToChatRef, U: ToUserId {
    GetChatMember {
      chat_id: chat.to_chat_ref(),
      user_id: user.to_user_id(),
    }
  }
}

/// Get information about a member of a chat.
pub trait CanGetChatMemberForChat {
  fn get_member<O>(&self, other: O) -> GetChatMember where O: ToUserId;
}

impl<C> CanGetChatMemberForChat for C where C: ToChatRef {
  fn get_member<O>(&self, other: O) -> GetChatMember where O: ToUserId {
    GetChatMember::new(self, other)
  }
}

/// Get information about a member of a chat.
pub trait CanGetChatMemberForUser {
  fn get_member_from<O>(&self, other: O) -> GetChatMember where O: ToChatRef;
}

impl<U> CanGetChatMemberForUser for U where U: ToUserId {
  fn get_member_from<O>(&self, other: O) -> GetChatMember where O: ToChatRef {
    GetChatMember::new(other, self)
  }
}
