extern crate error_chain_mini;
#[macro_use]
extern crate error_chain_mini_derive;
#[macro_use]
extern crate futures;
#[macro_use]
extern crate lazy_static;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde_value;
#[macro_use]
extern crate slog;
extern crate slog_term;

pub use self::telegram_bot::TelegramBot;

mod telegram_bot;
mod stream;
mod botrun;
mod tgfut;
mod botapi;
mod tglog;
mod advanced;
mod listener;

pub mod errors;
pub mod api;
pub mod types;
pub mod config;
pub mod vision;
