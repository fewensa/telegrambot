use crate::advanced::callback_query_handler::TGCallbackQueryHandler;
use crate::advanced::error_handler::TGErrorHandler;
use crate::advanced::message_handler::TGMessageHandler;
use crate::advanced::post_handler::TGPostHandler;
use crate::tglog;
use crate::types::{Update, UpdateKind};

pub struct TGAdvancedHandler {
  update: Update
}

impl TGAdvancedHandler {
  pub fn new(update: Update) -> Self {
    TGAdvancedHandler {
      update
    }
  }

  pub fn handle(&self) {
    debug!(tglog::advanced(), "UPDATE => {:?}", self.update);
    match &self.update.kind {
      UpdateKind::Message(message) => {
        TGMessageHandler::new(self.update.id, message, false)
          .handle()
      }
      UpdateKind::EditedMessage(message) => {
        TGMessageHandler::new(self.update.id, message, true)
          .handle()
      }
      UpdateKind::ChannelPost(post) => {
        TGPostHandler::new(self.update.id, post, false)
          .handle()
      }
      UpdateKind::EditedChannelPost(post) => {
        TGPostHandler::new(self.update.id, post, true)
          .handle()
      }
      UpdateKind::CallbackQuery(query) => {
        TGCallbackQueryHandler::new(self.update.id, query)
          .handle()
      }
      UpdateKind::Error(msg) => {
        TGErrorHandler::new(self.update.id, msg)
          .handle()
      }
      _ => {
        TGErrorHandler::new(self.update.id, &"Unknow update kind".to_string())
          .handle()
      }
    };
  }
}


