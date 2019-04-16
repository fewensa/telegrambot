use std::sync::Arc;

use crate::advanced::message_handler;
use crate::config::Config;
use crate::listener::Lout;
use crate::tglog;
use crate::types::{Chat, Forward, ForwardFrom, RawMessage, Update, UpdateKind};

pub struct TGAdvancedHandler {
  cfg: Arc<Config>,
  lout: Arc<Lout>,
}

impl TGAdvancedHandler {
  pub fn new(cfg: Arc<Config>, lout: Arc<Lout>) -> Self {
    TGAdvancedHandler { cfg, lout }
  }

  pub fn handle(&self, update: Update) {
    debug!(tglog::advanced(), "RAW MESSAGE: {:#?}", update);


    if let Some(update_listener) = self.lout.listen_update() {
      (*update_listener)(&update);
      return;
    }

    if let Some(raw) = &update.message {
      message_handler::handle(&self.lout, raw, update.is_edited);
      return;
    }
    if let Some(callback_query) = &update.callback_query {
      debug!(tglog::advanced(), "CALLBACK_QUERY: {:?}", callback_query);
    }
    if let Some(err) = &update.error {
      debug!(tglog::advanced(), "ERR: {:?}", err);
    }

  }
}
