use crate::{TGFuture, TGBotErrorKind};
use crate::types::Update;
use reqwest::r#async::Client;
use futures::future::Future;
use futures::stream::Stream;
use error_chain_mini::ErrorKind;

pub trait TGReq {
  type Resp: TGRet + 'static;
}

pub trait TGRet {

}

pub struct UpdateReq {}

impl TGReq for UpdateReq {
  type Resp = ();
}
