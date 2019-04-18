use crate::api::TGReq;
use crate::api::resp::RespType;
use crate::types::User;
use crate::errors::TGBotResult;
use crate::api::req::HttpReq;
use reqwest::Method;

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
pub struct GetMe;

impl TGReq for GetMe {
  type Resp = RespType<User>;

  fn request(&self) -> TGBotResult<HttpReq> {
    HttpReq::json_req(Method::GET, "getMe", self)
  }
}



