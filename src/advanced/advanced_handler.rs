use core::borrow::Borrow;
use std::sync::Arc;

use crate::advanced::message_handler;
use crate::api::BotApi;
use crate::config::Config;
use crate::listener::Lout;
use crate::tglog;
use crate::types::{RawMessage, Update, UpdateKind};
use crate::vision::{PossibilityMessage, VCallbackQuery, VMessagChat};

pub struct TGAdvancedHandler<'a> {
  lout: &'a Arc<Lout>,
  api: &'a BotApi,
}

impl<'a> TGAdvancedHandler<'a> {
  pub fn new(lout: &'a Arc<Lout>, api: &'a Arc<BotApi>) -> Self {
    TGAdvancedHandler {
      lout,
      api: api.borrow(),
    }
  }

  pub fn handle(&self, update: Update) {
    debug!(tglog::advanced(), "RAW MESSAGE: {:#?}", update);

    if let Some(update_listener) = self.lout.listen_update() {
      (*update_listener)((self.api, &update));
      return;
    }

    match &update.kind {
      UpdateKind::Message(ref raw) => {
        info!(tglog::advanced(), "{} | INCOMMING MESSAGE: {:?}", if update.is_edited { "EDITED" } else { "POST" }, raw);
        message_handler::handle(self.api, &self.lout, raw, update.is_edited);
        return;
      }
      UpdateKind::Channel(ref raw) => {
        info!(tglog::advanced(), "{} | INCOMMING CHANNEL MESSAGE: {:?}", if update.is_edited { "EDITED" } else { "POST" }, raw);
        message_handler::handle(self.api, &self.lout, raw, update.is_edited);
        return;
      }
      UpdateKind::CallbackQuery(ref callback_query) => {
        info!(tglog::advanced(), "INCOMMING CALLBACK_QUERY: {:?}", callback_query);
        if let Some(fnc) = self.lout.listen_callback_query() {
          let vcq = VCallbackQuery {
            id: callback_query.id.clone(),
            from: callback_query.from.clone(),
            message: PossibilityMessage::new(callback_query.message.clone()),
            chat_instance: callback_query.chat_instance.clone(),
            data: callback_query.data.clone(),
          };
          (*fnc)((self.api, &vcq));
          return;
        }
      }
      UpdateKind::Err(ref err) => {
        error!(tglog::advanced(), "Happen error for update: {:?}, you can post an issue to: https://github.com/fewensa/telegrambot/issues   (include error log)", err);
        error!(tglog::advanced(), "=====================================================");
        error!(tglog::advanced(), "ERROR LOG START");
        error!(tglog::advanced(), "=====================================================");
        error!(tglog::advanced(), "UPDATE: {:#?}", update);
        error!(tglog::advanced(), "=====================================================");
        error!(tglog::advanced(), "ERROR LOG END");
        error!(tglog::advanced(), "=====================================================");
        if let Some(fnc) = self.lout.listen_error() {
          (&fnc)((self.api, err));
          return;
        }
      }
      UpdateKind::Unknown => {
        let notice = "Not support update type. please post an issue to: https://github.com/fewensa/telegrambot/issues   (include error log)".to_string();
        error!(tglog::advanced(), "{:?}", notice);
        error!(tglog::advanced(), "=====================================================");
        error!(tglog::advanced(), "ERROR LOG START");
        error!(tglog::advanced(), "=====================================================");
        error!(tglog::advanced(), "UPDATE: {:#?}", update);
        error!(tglog::advanced(), "=====================================================");
        error!(tglog::advanced(), "ERROR LOG END");
        error!(tglog::advanced(), "=====================================================");
        if let Some(fnc) = self.lout.listen_error() {
          (&fnc)((self.api, &notice));
          return;
        }
      }
    };
  }
}

