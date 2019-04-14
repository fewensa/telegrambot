use std::sync::Arc;

use error_chain_mini::ErrorKind;
use futures::future::Future;
use futures::stream::Stream;

use crate::{Config, ConnectMode, TGBotErrorKind, TGBotResult, tglog};
use crate::advanced::TGAdvancedHandler;
use crate::stream::UpdatesStream;

pub fn run(cfg: Arc<Config>) -> TGBotResult<()> {
  match cfg.mode() {
    ConnectMode::Polling => self::polling(cfg),
    ConnectMode::Webhook => Err(TGBotErrorKind::ComingSoon.into_with(|| "Coming soon.."))
  }
}


fn polling(cfg: Arc<Config>) -> TGBotResult<()> {
  let stream = UpdatesStream::new(cfg);
  let future = stream.for_each(|update| {
    TGAdvancedHandler::new(update).handle();
    Ok(())
  }).map_err(|e| {
    // todo: some error handle.
    error!(tglog::telegram(), "Stream error: {:?}", e);
  });
  tokio::run(future);
  Ok(())
}
