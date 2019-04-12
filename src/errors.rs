use error_chain_mini::ChainedError;
use error_chain_mini::ErrorKind;

//#[derive(ErrorKind)]
pub enum TGBotErrorKind {
  //  #[msg(short = "Request error", detailed = "Request error")]
  ClientError(reqwest::Error),
  RequestError(reqwest::Error),
  ProxyError(reqwest::Error),
  LoseToken,
  ComingSoon,
  Other,
}

pub type TGBotError = ChainedError<TGBotErrorKind>;
pub type TGBotResult<T> = Result<T, TGBotError>;

// fixme: only development
impl ErrorKind for TGBotErrorKind {
  fn short(&self) -> &str {
    match self {
      _ => ""
    }
  }
}
