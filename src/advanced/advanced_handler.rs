use std::sync::Arc;

use crate::advanced::message_handler;
use crate::config::Config;
use crate::listener::Lout;
use crate::tglog;
use crate::types::{Update, UpdateKind, RawMessage};
use crate::vision::VMessagChat;

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

    match &update.kind {
      UpdateKind::Message(ref raw) => {
        message_handler::handle( &self.lout, raw, update.is_edited);
      },
      UpdateKind::Channel(ref raw) => {
        message_handler::handle(&self.lout, raw, update.is_edited);
      },
      UpdateKind::CallbackQuery(ref callback_query) => {
        debug!(tglog::advanced(), "CALLBACK_QUERY: {:?}", callback_query);
      },
      UpdateKind::Err(ref err) => {
        debug!(tglog::advanced(), "ERR: {:?}", err);
      },
      UpdateKind::Unknown => {
        error!(tglog::advanced(), "Not support update type.")
      }
    };

  }
}

