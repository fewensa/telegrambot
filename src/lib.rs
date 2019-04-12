extern crate error_chain_mini;
#[macro_use]
extern crate error_chain_mini_derive;
#[macro_use]
extern crate futures;

pub use self::errors::{TGBotError, TGBotErrorKind, TGBotResult};
pub use self::telegram_bot::{TelegramBot};
pub use self::types::*;
pub use self::tgfut::TGFuture;
pub use self::config::*;

mod telegram_bot;
mod errors;
mod stream;
mod botrun;
mod types;
mod tgfut;
mod boreq;
mod config;
