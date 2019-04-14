pub use self::get_updates::*;
pub use self::req::TGReq;
pub use self::resp::*;
pub use self::tgkit::TELEGRAM_API_URL;

mod req;
mod tgkit;
mod get_updates;
mod resp;
