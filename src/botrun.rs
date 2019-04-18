use std::sync::Arc;

use error_chain_mini::ErrorKind;
use futures::future::Future;
use futures::stream::Stream;

use crate::advanced::TGAdvancedHandler;
use crate::api::BotApi;
use crate::api::rawreq::RawReq;
use crate::config::{Config, ConnectMode};
use crate::errors::{TGBotErrorKind, TGBotResult};
use crate::listener::Lout;
use crate::stream::UpdatesStream;
use crate::tglog;

pub fn run(cfg: Config, lout: Arc<Lout>) -> TGBotResult<()> {
  match cfg.mode() {
    ConnectMode::Polling => self::polling(cfg, lout),
    ConnectMode::Webhook => Err(TGBotErrorKind::ComingSoon.into_with(|| "Coming soon.."))
  }
}


fn polling(cfg: Config, lout: Arc<Lout>) -> TGBotResult<()> {
  let token = cfg.token();
  let client = cfg.client();
  let rawreq = RawReq::new(Arc::new(client), token);
  let api = Arc::new(BotApi::new(rawreq));
  let stream = UpdatesStream::new(api.clone());

  let future = stream.for_each(move |update| {
    debug!(tglog::telegram(), "UPDATE: {:?}", update);
    TGAdvancedHandler::new(&lout, &api)
      .handle(update);
    Ok(())
  }).map_err(|e| {
    // todo: some error handle.
    error!(tglog::telegram(), "Stream error: {:?}", e);
  });
  tokio::run(future);
  Ok(())
}
