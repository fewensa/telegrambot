extern crate error_chain_mini;
#[macro_use]
extern crate error_chain_mini_derive;


pub use self::errors::{RBoteleError, RBoteleErrorKind, RBoteleResult};
pub use self::telegram_bot::TelegramBot;

mod telegram_bot;
mod errors;

