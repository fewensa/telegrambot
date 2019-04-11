use error_chain_mini::ErrorKind;
use futures::future::Future;
use futures::stream::Stream;

use crate::{Config, ConnectMode, TGBotErrorKind, TGBotResult};
use crate::stream::UpdatesStream;

pub fn run(cfg: &Config) -> TGBotResult<()> {
  match cfg.mode {
    ConnectMode::Polling => self::polling(cfg),
    ConnectMode::Webhook => Err(TGBotErrorKind::ComingSoon.into_with(|| "Coming soon.."))
  }
}


fn polling(cfg: &Config) -> TGBotResult<()> {
  let stream = UpdatesStream::new();
  let future = stream.for_each(|update| {
    println!("some update. {:?}", update);
    Ok(())
  }).map_err(|e| {
    // todo: some error.
    println!("{:?}", e);
  });
  tokio::run(future);
  Ok(())
}
