use crate::types::{Audio, Chat, Contact, Document, Forward, Location, PhotoSize, RawMessage, Sticker, User, Venue, Video, VideoNote, Voice, MessageEntity};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Message {
  /// Unique message identifier inside this chat.
  pub id: i64,
  /// Sender, can be empty for messages sent to channels.
  pub from: Option<User>,
  /// Date the message was sent in Unix time.
  pub date: i64,
  /// Conversation the message belongs to.
  pub chat: Chat,
  /// Information about the original message.
  pub forward: Option<Forward>,
  /// For replies, the original message. Note that the Message object in this field will not
  /// contain further reply_to_message fields even if it itself is a reply.
  pub reply_to_message: Option<Box<RawMessage>>,
  /// Date the message was last edited in Unix time.
  pub edit_date: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct VTextMessage {
  pub message: Message,
  pub text: String,
  pub entities: Vec<MessageEntity>
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct VAudioMessage {
  pub message: Message,
  pub audio: Audio,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct VDocumentMessage {
  pub message: Message,
  pub document: Document,
  pub caption: Option<String>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct VPhotoMessage {
  pub message: Message,
  pub photo: Vec<PhotoSize>,
  pub caption: Option<String>,
  pub media_group_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct VStickerMessage {
  pub message: Message,
  pub sticker: Sticker,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct VVideoMessage {
  pub message: Message,
  pub video: Video,
  pub caption: Option<String>,
  pub media_group_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct VVoiceMessage {
  pub message: Message,
  pub voice: Voice,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct VVideoNoteMessage {
  pub message: Message,
  pub video_note: VideoNote,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct VContactMessage {
  pub message: Message,
  pub contact: Contact,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct VLocationMessage {
  pub message: Message,
  pub location: Location,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct VVenueMessage {
  pub message: Message,
  pub venue: Venue,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct VNewChatMembersMessage {
  pub message: Message,
  pub members: Vec<User>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct VLeftChatMemberMessage {
  pub message: Message,
  pub member: User,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct VChatTitleMessage {
  pub message: Message,
  pub title: String,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct VChatPhotoMessage {
  pub message: Message,
  pub photos: Vec<PhotoSize>,
}

//pub struct VDeleteChatPhotoMessage {
//  pub message: Message,
//}
//
//pub struct VGroupChatCreatedMessage {
//  pub message: Message,
//}
//
//pub struct VSupergroupChatCreatedMessage {
//  pub message: Message,
//}
//
//pub struct VChannelChatCreatedMessage {
//  pub message: Message,
//}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct VMigrateToChatIdMessage {
  pub message: Message,
  pub migrate_to_chat_id: i64,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct VMigrateFromChatIdMessage {
  pub message: Message,
  pub migrate_from_chat_id: i64,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct VPinnedMessageMessage {
  pub message: Message,
  pub pinned: Box<RawMessage>,
}
