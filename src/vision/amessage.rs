use crate::types::*;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ATextMessage {
  /// Unique message identifier inside this chat.
  pub id: MessageId,
  /// Sender, can be empty for messages sent to channels.
  pub from: User,
  /// Date the message was sent in Unix time.
  pub date: i64,
  /// Conversation the message belongs to.
  pub chat: MessageChat,
  /// Information about the original message.
  pub forward: Option<Forward>,
  /// For replies, the original message. Note that the Message object in this field will not
  /// contain further reply_to_message fields even if it itself is a reply.
  pub reply_to_message: Option<Box<MessageOrChannelPost>>,
  /// Date the message was last edited in Unix time.
  pub edit_date: Option<i64>,
  /// Text content
  pub text: String,
  /// This object represents one special entity in a text message.
  /// For example, hashtags, usernames, URLs, etc.
  pub entities: Vec<MessageEntity>,
}

pub struct AChannelMessage {
  /// Unique message identifier inside this chat.
  pub id: MessageId,
  /// Date the message was sent in Unix time.
  pub date: i64,
  /// Conversation the message belongs to.
  pub chat: Channel,
  /// Information about the original message.
  pub forward: Option<Forward>,
  /// For replies, the original message. Note that the Message object in this field will not
  /// contain further reply_to_message fields even if it itself is a reply.
  pub reply_to_message: Option<Box<MessageOrChannelPost>>,
  /// Date the message was last edited in Unix time.
  pub edit_date: Option<i64>,
  /// Text content
  pub text: String,
  /// This object represents one special entity in a text message.
  /// For example, hashtags, usernames, URLs, etc.
  pub entities: Vec<MessageEntity>,
}



