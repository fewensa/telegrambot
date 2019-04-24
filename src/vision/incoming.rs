use crate::types::{CallbackQuery, Update, UpdateKind, User};
use crate::vision::possibility::PossibilityMessage;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Incoming {
  update: Update
}


impl Incoming {
  pub(crate) fn new(update: Update) -> Self {
    Self {
      update
    }
  }

  pub fn from(&self) -> Option<User> {
    match self.update.clone().kind {
      UpdateKind::CallbackQuery(cq) => Some(cq.from),
      UpdateKind::Channel(raw) => raw.from,
      UpdateKind::Message(raw) => raw.from,
      _ => None
    }
  }

  pub fn is_edited(&self) -> bool {
    self.update.is_edited
  }

  pub fn is_message(&self) -> bool {
    match self.update.kind {
      UpdateKind::Message(_) => true,
      _ => false
    }
  }

  pub fn is_channel(&self) -> bool {
    match self.update.kind {
      UpdateKind::Channel(_) => true,
      _ => false
    }
  }

  pub fn is_callback_query(&self) -> bool {
    match self.update.kind {
      UpdateKind::CallbackQuery(_) => true,
      _ => false
    }
  }

  pub fn possibility(&self) -> Option<PossibilityMessage> {
    match self.update.clone().kind {
      UpdateKind::Message(raw) => Some(PossibilityMessage::new(raw)),
      UpdateKind::Channel(raw) => Some(PossibilityMessage::new(raw)),
      _ => None
    }
  }

  pub fn callback_query(&self) -> Option<CallbackQuery> {
    match self.update.clone().kind {
      UpdateKind::CallbackQuery(c) => Some(c),
      _ => None
    }
  }
}

