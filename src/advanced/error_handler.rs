use std::sync::Arc;

use crate::listener::Lout;

pub struct TGErrorHandler<'a> {
  update_id: i64,
  msg: &'a String
}

impl<'a> TGErrorHandler<'a> {
  pub fn new(update_id: i64, msg: &'a String) -> Self {
    TGErrorHandler { update_id, msg }
  }

  pub fn handle(&self, lout: &Arc<Lout>) {

  }
}
