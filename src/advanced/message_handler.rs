use std::sync::Arc;

use crate::advanced::text_handler;
use crate::listener::Lout;
use crate::types::{Message, MessageKind};

pub struct TGMessageHandler<'a> {
  update_id: i64,
  message: &'a Message,
  edited: bool,
}

impl<'a> TGMessageHandler<'a> {
  pub fn new(update_id: i64, message: &'a Message, edited: bool) -> Self {
    TGMessageHandler {
      update_id,
      message,
      edited,
    }
  }

  pub fn handle(&self, lout: &Arc<Lout>) {
    match self.message.kind {
      MessageKind::Text { ref data, ref entities } => {
        text_handler::handle_message(self.update_id, self.edited, self.message, data, entities, lout)
      }
      MessageKind::Audio { ref data, .. } => {}
      MessageKind::Document { ref data, ref caption } => {}
      MessageKind::Photo { ref data, ref caption, ref media_group_id } => {}
      MessageKind::Sticker { ref data, .. } => {}
      MessageKind::Video { ref data, ref caption, ref media_group_id } => {}
      MessageKind::Voice { ref data, .. } => {}
      MessageKind::VideoNote { ref data, .. } => {}
      MessageKind::Contact { ref data, .. } => {}
      MessageKind::Location { ref data, .. } => {}
      MessageKind::Venue { ref data, .. } => {}
      MessageKind::NewChatMembers { ref data, .. } => {}
      MessageKind::LeftChatMember { ref data, .. } => {}
      MessageKind::NewChatTitle { ref data, .. } => {}
      MessageKind::NewChatPhoto { ref data, .. } => {}
      MessageKind::DeleteChatPhoto => {}
      MessageKind::GroupChatCreated => {}
      MessageKind::SupergroupChatCreated => {}
      MessageKind::ChannelChatCreated => {}
      MessageKind::MigrateToChatId { ref data } => {}
      MessageKind::MigrateFromChatId { ref data } => {}
      MessageKind::PinnedMessage { ref data } => {}
      MessageKind::Unknown { ref raw } => {}
    };
  }
}
