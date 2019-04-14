extern crate error_chain_mini;
#[macro_use]
extern crate error_chain_mini_derive;
#[macro_use]
extern crate futures;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde_value;
#[macro_use]
extern crate slog;
extern crate slog_term;

#[macro_use]
extern crate lazy_static;

pub use self::config::*;
pub use self::errors::{TGBotError, TGBotErrorKind, TGBotResult};
pub use self::telegram_bot::{TelegramBot, Track};
pub use self::tgfut::TGFuture;
pub use self::types::*;

mod telegram_bot;
mod errors;
mod stream;
mod botrun;
mod types;
mod tgfut;
mod botapi;
mod config;
mod tglog;
mod advanced;
