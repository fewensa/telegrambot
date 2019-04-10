use error_chain_mini::ChainedError;
use error_chain_mini::ErrorKind;

#[derive(ErrorKind)]
pub enum RBoteleErrorKind {
  Other
}

pub type RBoteleError = ChainedError<RBoteleErrorKind>;
pub type RBoteleResult<T> = Result<T, RBoteleError>;


