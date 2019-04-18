use reqwest::Method;

use crate::api::req::HttpReq;
use crate::api::resp::RespType;
use crate::api::TGReq;
use crate::errors::TGBotResult;
use crate::types::*;

/// Use this method to get a list of profile pictures for a user.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct GetUserProfilePhotos {
  user_id: UserId,
  offset: Option<i64>,
  limit: Option<i64>,
}


impl TGReq for GetUserProfilePhotos {
  type Resp = RespType<UserProfilePhotos>;

  fn request(&self) -> TGBotResult<HttpReq> {
    HttpReq::json_req(Method::POST, "getUserProfilePhotos", self)
  }
}


impl GetUserProfilePhotos {
  pub fn new<U>(user: U) -> Self where U: ToUserId {
    GetUserProfilePhotos {
      user_id: user.to_user_id(),
      offset: None,
      limit: None,
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
}

/// Get a list of profile pictures for a user.
pub trait CanGetUserProfilePhotos {
  fn get_user_profile_photos(&self) -> GetUserProfilePhotos;
}

impl<'b, U> CanGetUserProfilePhotos for U where U: ToUserId {
  fn get_user_profile_photos(&self) -> GetUserProfilePhotos {
    GetUserProfilePhotos::new(self)
  }
}
