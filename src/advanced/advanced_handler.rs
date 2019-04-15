use std::sync::Arc;

use crate::advanced::callback_query_handler::TGCallbackQueryHandler;
use crate::advanced::channel_post_handler::TGChannelPostHandler;
use crate::advanced::error_handler::TGErrorHandler;
use crate::advanced::message_handler::TGMessageHandler;
use crate::config::Config;
use crate::listener::Lout;
use crate::tglog;
use crate::types::{Update, UpdateKind};

pub struct TGAdvancedHandler {
  cfg: Arc<Config>,
  lout: Arc<Lout>,
  update: Update,
}

impl TGAdvancedHandler {
  pub fn new(cfg: Arc<Config>, lout: Arc<Lout>, update: Update) -> Self {
    TGAdvancedHandler {
      cfg,
      lout,
      update,
    }
  }

  pub fn handle(&self) {
    debug!(tglog::advanced(), "UPDATE => {:?}", self.update);

    if let Some(update_listener) = self.lout.listen_update() {
      (*update_listener)(&self.update);
      return;
    }

    match &self.update.kind {
      UpdateKind::Message(message) => {
        TGMessageHandler::new(self.update.id, message, false)
          .handle(&self.lout)
      }
      UpdateKind::EditedMessage(message) => {
        TGMessageHandler::new(self.update.id, message, true)
          .handle(&self.lout)
      }
      UpdateKind::ChannelPost(post) => {
        TGChannelPostHandler::new(self.update.id, post, false)
          .handle(&self.lout)
      }
      UpdateKind::EditedChannelPost(post) => {
        TGChannelPostHandler::new(self.update.id, post, true)
          .handle(&self.lout)
      }
      UpdateKind::CallbackQuery(query) => {
        TGCallbackQueryHandler::new(self.update.id, query)
          .handle(&self.lout)
      }
      UpdateKind::Error(msg) => {
        TGErrorHandler::new(self.update.id, msg)
          .handle(&self.lout)
      }
      _ => {
        TGErrorHandler::new(self.update.id, &"Unknow update kind".to_string())
          .handle(&self.lout)
      }
    };
  }
}


