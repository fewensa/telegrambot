use std::sync::Arc;

use crate::listener::Lout;
use crate::types::CallbackQuery;

pub struct TGCallbackQueryHandler<'a> {
  update_id: i64,
  callback_query: &'a CallbackQuery,
}

impl<'a> TGCallbackQueryHandler<'a> {
  pub fn new(update_id: i64, callback_query: &'a CallbackQuery) -> Self {
    TGCallbackQueryHandler {
      update_id,
      callback_query,
    }
  }

  pub fn handle(&self, lout: &Arc<Lout>) {

  }
}

