use crate::types::{ChannelPost, MessageKind};

pub struct TGPostHandler<'a> {
  update_id: i64,
  post: &'a ChannelPost,
  edited: bool
}

impl<'a> TGPostHandler<'a> {
  pub fn new(update_id: i64, post: &'a ChannelPost, edited: bool) -> Self {
    TGPostHandler {
      update_id,
      post,
      edited
    }
  }

  pub fn handle(&self) {
    match self.post.kind {
      MessageKind::Text { ref data, ref entities } => {}
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
