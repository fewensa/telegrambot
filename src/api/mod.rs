pub use self::get_updates::*;
pub use self::req::TGReq;
pub use self::resp::{RespParas, TGResp};
pub use self::tgkit::TELEGRAM_API_URL;
pub use self::botapi::*;

mod botapi;
mod req;
mod tgkit;
mod resp;
mod get_me;
mod get_updates;
