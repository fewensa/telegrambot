use error_chain_mini::ChainedError;
use error_chain_mini::ErrorKind;

use crate::api::RespParas;

#[derive(ErrorKind)]
pub enum TGBotErrorKind {
  #[msg(short = "Client error", detailed = "inner: {:?}", _0)]
  ClientError(reqwest::Error),
  #[msg(short = "Request error", detailed = "inner: {:?}", _0)]
  RequestError(reqwest::Error),
  #[msg(short = "Request error", detailed = "inner: {:?}", _0)]
  ProxyError(reqwest::Error),
  #[msg(short = "Json error", detailed = "inner: {:?}", _0)]
  JsonError(::serde_json::Error),
  #[msg(short = "Call telegram error", detailed = "inner: {:?} => {:?}", _0, _1)]
  TGError(String, Option<RespParas>),
//  CanotSendMessage(&'static str),
  EmptyBody,
  LoseToken,
  ComingSoon,
  Other,
}

pub type TGBotError = ChainedError<TGBotErrorKind>;
pub type TGBotResult<T> = Result<T, TGBotError>;

//// todox: only development
//impl ErrorKind for TGBotErrorKind {
//  fn short(&self) -> &str {
//    match self {
//      _ => ""
//    }
//  }
//}
