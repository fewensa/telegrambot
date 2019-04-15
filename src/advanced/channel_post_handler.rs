use crate::advanced::text_handler;
use crate::types::{ChannelPost, MessageKind};
use std::sync::Arc;
use crate::listener::Lout;

pub struct TGChannelPostHandler<'a> {
  update_id: i64,
  channel_post: &'a ChannelPost,
  edited: bool
}

impl<'a> TGChannelPostHandler<'a> {
  pub fn new(update_id: i64, channel_post: &'a ChannelPost, edited: bool) -> Self {
    TGChannelPostHandler {
      update_id,
      channel_post,
      edited
    }
  }

  pub fn handle(&self, lout: &Arc<Lout>) {
    match self.channel_post.kind {
      MessageKind::Text { ref data, ref entities } => {
        text_handler::handle_channel_post(self.update_id, self.edited, self.channel_post, data, entities, lout)
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
    }
  }
}
