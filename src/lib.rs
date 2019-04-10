extern crate error_chain_mini;
#[macro_use]
extern crate error_chain_mini_derive;
#[macro_use]
extern crate futures;

pub use self::errors::{TGBotError, TGBotErrorKind, TGBotResult};
pub use self::telegram_bot::{Config, ConnectMode, TelegramBot};
pub use self::types::*;

mod telegram_bot;
mod errors;
mod stream;
mod botrun;
mod types;
