use std::sync::Arc;

use reqwest::Method;

use crate::api::req::HttpReq;
use crate::api::resp::{HttpResp, RespType};
use crate::api::TGReq;
use crate::config::Config;
use crate::errors::TGBotResult;
use crate::tgfut::TGFuture;
use crate::types::User;

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
pub struct GetMe;

impl TGReq for GetMe {
  type Resp = RespType<User>;

  fn request(&self) -> TGBotResult<HttpReq> {
    HttpReq::json_req(Method::GET, "getMe", self)
  }
}



