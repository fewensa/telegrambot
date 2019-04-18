pub use self::botapi::{BotApi, BotFutApi, TELEGRAM_API_URL};
pub use self::get_updates::*;
pub use self::req::TGReq;
pub use self::resp::{RespParas, TGResp};

pub mod rawreq;

mod botapi;
mod req;
mod resp;
mod get_me;
mod get_updates;
//mod tgkit;
