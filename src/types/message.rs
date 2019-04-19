use serde::de::{Deserialize, Deserializer, Error};

use crate::api;
use crate::types::*;



/// Information about the original message.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Forward {
  /// Date the original message was sent in Unix time
  pub date: i64,
  /// Sender of the original message.
  pub from: ForwardFrom,
}

/// Information about the source of the original message.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum ForwardFrom {
  /// Sender of the original message.
  User {
    /// Sender of the original message.
    user: User,
  },
  /// For messages forwarded from a channel, information about the original channel.
  Channel {
    /// Original channel.
    channel: Channel,
    /// Identifier of the original message in the channel
    message_id: i64,
  },
  ChannelHiddenUser {
    sender_name: String
  },
}

/// This object represents a message. Directly mapped.
#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct RawMessage {
  /// Unique message identifier inside this chat.
  pub message_id: i64,
  /// Sender, can be empty for messages sent to channels.
  pub from: Option<User>,
  /// Date the message was sent in Unix time.
  pub date: i64,
  /// Conversation the message belongs to.
  pub chat: Chat,
  /// For forwarded messages, sender of the original message.
  pub forward_from: Option<User>,
  /// For messages forwarded from a channel, information about the original channel.
  pub forward_from_chat: Option<Chat>,
  /// For forwarded channel posts, identifier of the original message in the channel.
  pub forward_from_message_id: Option<i64>,
  /// For forwarded messages, date the original message was sent in Unix time.
  pub forward_date: Option<i64>,
  /// For replies, the original message. Note that the Message object in this field will not
  /// contain further reply_to_message fields even if it itself is a reply.
  pub reply_to_message: Option<Box<RawMessage>>,
  /// Date the message was last edited in Unix time.
  pub edit_date: Option<i64>,
  /// The unique identifier of a media message group this message belongs to.
  pub media_group_id: Option<String>,
  /// For text messages, the actual UTF-8 text of the message, 0-4096 characters.
  pub text: Option<String>,
  /// For text messages, special entities like usernames, URLs, bot commands, etc.
  /// that appear in the text.
  pub entities: Option<Vec<MessageEntity>>,
  /// Message is an audio file, information about the file.
  pub audio: Option<Audio>,
  /// Message is a general file, information about the file.
  pub document: Option<Document>,
  // pub game: Option<Game>,
  /// Message is a photo, available sizes of the photo.
  pub photo: Option<Vec<PhotoSize>>,
  /// Message is a sticker, information about the sticker.
  pub sticker: Option<Sticker>,
  /// Message is a video, information about the video.
  pub video: Option<Video>,
  /// Message is a voice message, information about the file.
  pub voice: Option<Voice>,
  /// Message is a video note message, information about the file.
  pub video_note: Option<VideoNote>,
  /// Caption for the document, photo or video, 0-200 characters.
  pub caption: Option<String>,
  /// Message is a shared contact, information about the contact.
  pub contact: Option<Contact>,
  /// Message is a shared location, information about the location.
  pub location: Option<Location>,
  /// Message is a venue, information about the venue.
  pub venue: Option<Venue>,
  /// New members that were added to the group or supergroup and information
  /// about them (the bot itself may be one of these members)
  pub new_chat_members: Option<Vec<User>>,
  /// A member was removed from the group, information about
  /// them (this member may be the bot itself)
  pub left_chat_member: Option<User>,
  /// A chat title was changed to this value.
  pub new_chat_title: Option<String>,
  /// A chat photo was change to this value.
  pub new_chat_photo: Option<Vec<PhotoSize>>,
  /// Service message: the chat photo was deleted.
  pub delete_chat_photo: Option<True>,
  /// Service message: the group has been created.
  pub group_chat_created: Option<True>,
  /// Service message: the supergroup has been created. This field can‘t be received in a
  /// message coming through updates, because bot can’t be a member of a supergroup when
  /// it is created. It can only be found in reply_to_message if someone replies to a very
  /// first message in a directly created supergroup.
  pub supergroup_chat_created: Option<True>,
  /// Service message: the channel has been created. This field can‘t be received in a message
  /// coming through updates, because bot can’t be a member of a channel when it is created.
  /// It can only be found in reply_to_message if someone replies
  /// to a very first message in a channel.
  pub channel_chat_created: Option<True>,
  /// The group has been migrated to a supergroup with the specified identifier.
  pub migrate_to_chat_id: Option<i64>,
  /// The supergroup has been migrated from a group with the specified identifier.
  pub migrate_from_chat_id: Option<i64>,
  /// Specified message was pinned. Note that the Message object in this field will not contain
  /// further reply_to_message fields even if it is itself a reply.
  pub pinned_message: Option<Box<RawMessage>>,
  /// Forward from channel by a hidden user.
  pub forward_sender_name: Option<String>,
}

/// This object represents one special entity in a text message.
/// For example, hashtags, usernames, URLs, etc.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct MessageEntity {
  /// Offset in UTF-16 code units to the start of the entity
  pub offset: i64,
  /// Length of the entity in UTF-16 code units
  pub length: i64,
  /// Kind of the entity.
  pub kind: MessageEntityKind,
}

/// Kind of the entity.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum MessageEntityKind {
  Mention,
  Hashtag,
  BotCommand,
  Url,
  Email,
  Bold,
  Italic,
  Code,
  Pre,
  TextLink(String),
  // TODO(knsd) URL?
  TextMention(User),
  #[doc(hidden)]
  Unknown(RawMessageEntity),
}

impl<'de> Deserialize<'de> for MessageEntity {
  fn deserialize<D>(deserializer: D) -> Result<MessageEntity, D::Error>
    where D: Deserializer<'de>
  {
    use self::MessageEntityKind::*;

    let raw: RawMessageEntity = Deserialize::deserialize(deserializer)?;

    let offset = raw.offset;
    let length = raw.length;

    macro_rules! required_field {
      ($name:ident) => {{
        match raw.$name {
          Some(val) => val,
          None => return Err(D::Error::missing_field(stringify!($name)))
        }
      }}
    }

    let kind = match raw.type_.as_str() {
      "mention" => Mention,
      "hashtag" => Hashtag,
      "bot_command" => BotCommand,
      "url" => Url,
      "email" => Email,
      "bold" => Bold,
      "italic" => Italic,
      "code" => Code,
      "pre" => Pre,
      "text_link" => TextLink(required_field!(url)),
      "text_mention" => TextMention(required_field!(user)),
      _ => Unknown(raw),
    };

    Ok(MessageEntity {
      offset,
      length,
      kind,
    })
  }
}

/// This object represents one special entity in a text message.
/// For example, hashtags, usernames, URLs, etc. Directly mapped.
#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct RawMessageEntity {
  /// Type of the entity. Can be mention (@username), hashtag, bot_command, url, email,
  /// bold (bold text), italic (italic text), code (monowidth string), pre (monowidth block),
  /// text_link (for clickable text URLs), text_mention (for users without usernames).
  #[serde(rename = "type")]
  pub type_: String,
  /// Offset in UTF-16 code units to the start of the entity.
  pub offset: i64,
  /// Length of the entity in UTF-16 code units.
  pub length: i64,
  /// For “text_link” only, url that will be opened after user taps on the text.
  pub url: Option<String>,
  /// For “text_mention” only, the mentioned user.
  pub user: Option<User>,
}

/// This object represents one size of a photo or a file / sticker thumbnail.
#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct PhotoSize {
  /// Unique identifier for this file.
  pub file_id: String,
  /// Photo width.
  pub width: i64,
  /// Photo height.
  pub height: i64,
  /// File size.
  pub file_size: Option<i64>,
}

/// This object represents an audio file to be treated as music by the Telegram clients.
#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct Audio {
  /// Unique identifier for this file.
  pub file_id: String,
  /// Duration of the audio in seconds as defined by sender.
  pub duration: i64,
  /// Performer of the audio as defined by sender or by audio tags.
  pub performer: Option<String>,
  /// Title of the audio as defined by sender or by audio tags.
  pub title: Option<String>,
  /// MIME type of the file as defined by sender.
  pub mime_type: Option<String>,
  /// File size.
  pub file_size: Option<i64>,
}

/// This object represents a general file (as opposed to photos, voice messages and audio files).
#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct Document {
  /// Unique file identifier.
  pub file_id: String,
  /// Document thumbnail as defined by sender.
  pub thumb: Option<PhotoSize>,
  /// Original filename as defined by sender.
  pub file_name: Option<String>,
  /// MIME type of the file as defined by sender.
  pub mime_type: Option<String>,
  /// File size.
  pub file_size: Option<i64>,
}

/// This object represents a sticker.
#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct Sticker {
  /// Unique identifier for this file.
  pub file_id: String,
  /// Sticker width.
  pub width: i64,
  /// Sticker height.
  pub height: i64,
  /// Sticker thumbnail in .webp or .jpg format.
  pub thumb: Option<PhotoSize>,
  /// Emoji associated with the sticker.
  pub emoji: Option<String>,
  /// File size.
  pub file_size: Option<i64>,
}

/// This object represents a video file.
#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct Video {
  /// Unique identifier for this file.
  pub file_id: String,
  /// Video width as defined by sender.
  pub width: i64,
  /// Video height as defined by sender.
  pub height: i64,
  /// Duration of the video in seconds as defined by sender.
  pub duration: i64,
  /// Video thumbnail.
  pub thumb: Option<PhotoSize>,
  /// Mime type of a file as defined by sender.
  pub mime_type: Option<String>,
  /// File size.
  pub file_size: Option<i64>,
}

/// This object represents a voice note.
#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct Voice {
  /// Unique identifier for this file.
  pub file_id: String,
  /// Duration of the audio in seconds as defined by sender.
  pub duration: i64,
  /// MIME type of the file as defined by sender.
  pub mime_type: Option<String>,
  /// File size.
  pub file_size: Option<i64>,
}

/// This object represents a video message (available in Telegram apps as of v.4.0).
#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct VideoNote {
  /// Unique identifier for this file.
  pub file_id: String,
  pub length: i64,
  /// Duration of the video in seconds as defined by sender.
  pub duration: i64,
  /// Video thumbnail.
  pub thumb: Option<PhotoSize>,
  /// File size.
  pub file_size: Option<i64>,
}

/// This object represents a phone contact.
#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct Contact {
  /// Contact's phone number.
  pub phone_number: String,
  /// Contact's first name.
  pub first_name: String,
  /// Contact's last name.
  pub last_name: Option<String>,
  /// Contact's user identifier in Telegram.
  pub user_id: Option<i64>,
}

/// This object represents a point on the map.
#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct Location {
  /// Longitude as defined by sender.
  pub longitude: f32,
  /// Latitude as defined by sender.
  pub latitude: f32,
}

/// This object represents a venue.
#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct Venue {
  /// Venue location.
  pub location: Location,
  /// Name of the venue.
  pub title: String,
  /// Address of the venue.
  pub address: String,
  /// Foursquare identifier of the venue.
  pub foursquare_id: Option<String>,
}

/// This object represent a user's profile pictures.
#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct UserProfilePhotos {
  /// Total number of profile pictures the target user has.
  pub total_count: i64,
  /// Requested profile pictures (in up to 4 sizes each).
  pub photos: Vec<Vec<PhotoSize>>,
}

/// This object represents a file ready to be downloaded.
/// The file can be downloaded via the link `https://api.telegram.org/file/bot<token>/<file_path>`.
/// It is guaranteed that the link will be valid for at least 1 hour.
#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct File {
  /// Unique identifier for this file.
  pub file_id: String,
  /// File size, if known.
  pub file_size: Option<i64>,
  /// File path. Use `https://api.telegram.org/file/bot<token>/<file_path>` to get the file.
  pub file_path: Option<String>,
}

impl File {
  pub fn get_url(&self, token: &str) -> Option<String> {
    self.file_path.as_ref().map(|path| format!("{}/file/bot{}/{}", api::telegram_api_url(), token, path))
  }
}

/// Strongly typed ParseMode.
/// See [documentation](https://core.telegram.org/bots/api#formatting-options) for details.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize)]
pub enum ParseMode {
  /// Use markdown formatting.
  Markdown,
  /// Use HTML formatting.
  #[serde(rename = "HTML")]
  Html,
}
