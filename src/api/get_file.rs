use reqwest::Method;

use crate::api::req::HttpReq;
use crate::api::resp::RespType;
use crate::api::TGReq;
use crate::errors::TGBotResult;
use crate::types::*;
use crate::vision::PossibilityMessage;

/// Use this method to get basic info about a file and prepare it for downloading.
/// For the moment, bots can download files of up to 20MB in size.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct GetFile {
  file_id: FileRef,
}


impl TGReq for GetFile {
  type Resp = RespType<File>;

  fn request(&self) -> TGBotResult<HttpReq> {
    HttpReq::json_req(Method::POST, "getFile", self)
  }
}


impl GetFile {
  pub fn new<F>(file: F) -> Self where F: ToFileRef {
    Self {
      file_id: file.to_file_ref()
    }
  }
}

/// Get basic info about a file and prepare it for downloading.
pub trait CanGetFile {
  fn get_file(&self) -> GetFile;
}

impl<F> CanGetFile for F where F: ToFileRef {
  fn get_file(&self) -> GetFile {
    GetFile::new(self)
  }
}
