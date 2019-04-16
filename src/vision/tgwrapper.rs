//use crate::types::*;
//
///// This object represents an incoming update.
//#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
//pub struct WUpdate {
//  /// The update‘s unique identifier. Update identifiers start from a certain
//  /// positive number and increase sequentially.
//  #[serde(rename = "update_id")]
//  pub id: i64,
//  pub message: WRawMessage
//}
//
//
//
//
///// This object represents a message. Directly mapped.
//#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
//pub struct WRawMessage {
//  /// Unique message identifier inside this chat.
//  pub message_id: i64,
//  /// Sender, can be empty for messages sent to channels.
//  pub from: Option<User>,
//  /// Date the message was sent in Unix time.
//  pub date: i64,
//  /// Conversation the message belongs to.
//  pub chat: Chat,
//  /// For forwarded messages, sender of the original message.
//  pub forward_from: Option<User>,
//  /// For messages forwarded from a channel, information about the original channel.
//  pub forward_from_chat: Option<Chat>,
//  /// For forwarded channel posts, identifier of the original message in the channel.
//  pub forward_from_message_id: Option<i64>,
//  /// For forwarded messages, date the original message was sent in Unix time.
//  pub forward_date: Option<i64>,
//  /// For replies, the original message. Note that the Message object in this field will not
//  /// contain further reply_to_message fields even if it itself is a reply.
//  pub reply_to_message: Option<Box<WRawMessage>>,
//  /// Date the message was last edited in Unix time.
//  pub edit_date: Option<i64>,
//  /// The unique identifier of a media message group this message belongs to.
//  pub media_group_id: Option<String>,
//  /// For text messages, the actual UTF-8 text of the message, 0-4096 characters.
//  pub text: Option<String>,
//  /// For text messages, special entities like usernames, URLs, bot commands, etc.
//  /// that appear in the text.
//  pub entities: Option<Vec<MessageEntity>>,
//  /// Message is an audio file, information about the file.
//  pub audio: Option<Audio>,
//  /// Message is a general file, information about the file.
//  pub document: Option<Document>,
//  // pub game: Option<Game>,
//  /// Message is a photo, available sizes of the photo.
//  pub photo: Option<Vec<PhotoSize>>,
//  /// Message is a sticker, information about the sticker.
//  pub sticker: Option<Sticker>,
//  /// Message is a video, information about the video.
//  pub video: Option<Video>,
//  /// Message is a voice message, information about the file.
//  pub voice: Option<Voice>,
//  /// Message is a video note message, information about the file.
//  pub video_note: Option<VideoNote>,
//  /// Caption for the document, photo or video, 0-200 characters.
//  pub caption: Option<String>,
//  /// Message is a shared contact, information about the contact.
//  pub contact: Option<Contact>,
//  /// Message is a shared location, information about the location.
//  pub location: Option<Location>,
//  /// Message is a venue, information about the venue.
//  pub venue: Option<Venue>,
//  /// New members that were added to the group or supergroup and information
//  /// about them (the bot itself may be one of these members)
//  pub new_chat_members: Option<Vec<User>>,
//  /// A member was removed from the group, information about
//  /// them (this member may be the bot itself)
//  pub left_chat_member: Option<User>,
//  /// A chat title was changed to this value.
//  pub new_chat_title: Option<String>,
//  /// A chat photo was change to this value.
//  pub new_chat_photo: Option<Vec<PhotoSize>>,
//  /// Service message: the chat photo was deleted.
//  pub delete_chat_photo: Option<True>,
//  /// Service message: the group has been created.
//  pub group_chat_created: Option<True>,
//  /// Service message: the supergroup has been created. This field can‘t be received in a
//  /// message coming through updates, because bot can’t be a member of a supergroup when
//  /// it is created. It can only be found in reply_to_message if someone replies to a very
//  /// first message in a directly created supergroup.
//  pub supergroup_chat_created: Option<True>,
//  /// Service message: the channel has been created. This field can‘t be received in a message
//  /// coming through updates, because bot can’t be a member of a channel when it is created.
//  /// It can only be found in reply_to_message if someone replies
//  /// to a very first message in a channel.
//  pub channel_chat_created: Option<True>,
//  /// The group has been migrated to a supergroup with the specified identifier.
//  pub migrate_to_chat_id: Option<i64>,
//  /// The supergroup has been migrated from a group with the specified identifier.
//  pub migrate_from_chat_id: Option<i64>,
//  /// Specified message was pinned. Note that the Message object in this field will not contain
//  /// further reply_to_message fields even if it is itself a reply.
//  pub pinned_message: Option<Box<WRawMessage>>,
//  /// Forward from channel by a hidden user.
//  pub forward_sender_name: Option<String>,
//}
//
