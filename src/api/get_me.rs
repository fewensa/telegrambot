//use std::sync::Arc;
//
//use crate::api::resp::{HttpResp, RespType};
//use crate::api::{TGReq, tgkit};
//use crate::config::Config;
//use crate::tgfut::TGFuture;
//use crate::types::User;
//
//#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
//pub struct GetMe;
//
//impl TGReq for GetMe {
//  type Resp = RespType<Vec<User>>;
//
//  fn request(&self, cfg: Arc<Config>) -> TGFuture<HttpResp> {
//    tgkit::exec(cfg, "getMe", self)
//  }
//}
//
//
//
