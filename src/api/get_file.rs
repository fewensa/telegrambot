use reqwest::Method;

use crate::api::req::HttpReq;
use crate::api::resp::RespType;
use crate::api::TGReq;
use crate::errors::TGBotResult;
use crate::types::*;
use crate::vision::PossibilityMessage;
use std::borrow::Cow;

/// Use this method to get basic info about a file and prepare it for downloading.
/// For the moment, bots can download files of up to 20MB in size.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct GetFile<'a> {
  file_id: Cow<'a, str>,
}


impl<'a> TGReq for GetFile<'a> {
  type Resp = RespType<File>;

  fn request(&self) -> TGBotResult<HttpReq> {
    HttpReq::json_req(Method::POST, "getFile", self)
  }
}


impl<'a> GetFile<'a> {
  pub fn new<F>(file: F) -> Self where F: Into<Cow<'a, str>> {
    Self {
      file_id: file.into()
    }
  }
}
