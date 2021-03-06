use serde::de::{Deserialize, Deserializer, Error};

/// This object represents a Telegram user or bot.
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Deserialize)]
pub struct User {
  /// Unique identifier for this user or bot.
  pub id: i64,
  /// User‘s or bot’s first name.
  pub first_name: String,
  /// User‘s or bot’s last name.
  pub last_name: Option<String>,
  /// User‘s or bot’s username.
  pub username: Option<String>,
  /// User is bot
  pub is_bot: Option<bool>,
  // language_code
}

/// This object represents a group.
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Deserialize)]
pub struct Group {
  /// Unique identifier for this chat.
  pub id: i64,
  /// Title, for supergroups, channels and group chats.
  pub title: String,
  /// True if a group has ‘All Members Are Admins’ enabled.
  pub all_members_are_administrators: bool,
}

/// This object represents a supergroup.
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Deserialize)]
pub struct Supergroup {
  /// Unique identifier for this chat.
  pub id: i64,
  /// Title, for supergroups, channels and group chats.
  pub title: String,
  /// Username for supergroup.
  pub username: Option<String>,
}

/// This object represents a channel.
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Deserialize)]
pub struct Channel {
  /// Unique identifier for this chat.
  pub id: i64,
  /// Title, for supergroups, channels and group chats.
  pub title: String,
  /// Username for channel.
  pub username: Option<String>,
}

/// This object represents a private, group or supergroup.
// todo: necessity
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum MessageChat {
  Private(User),
  Group(Group),
  Supergroup(Supergroup),
  #[doc(hidden)]
  Unknown(RawChat),
}

impl MessageChat {
  pub fn id(&self) -> i64 {
    match *self {
      MessageChat::Private(ref x) => x.id,
      MessageChat::Group(ref x) => x.id,
      MessageChat::Supergroup(ref x) => x.id,
      MessageChat::Unknown(ref x) => x.id,
    }
  }
}

/// This object represents a chat.
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum Chat {
  Private(User),
  Group(Group),
  Supergroup(Supergroup),
  Channel(Channel),
  #[doc(hidden)]
  Unknown(RawChat),
}

impl Chat {
  pub fn id(&self) -> i64 {
    match *self {
      Chat::Private(ref x) => x.id.into(),
      Chat::Group(ref x) => x.id.into(),
      Chat::Supergroup(ref x) => x.id.into(),
      Chat::Channel(ref x) => x.id.into(),
      Chat::Unknown(ref x) => x.id.into(),
    }
  }
}

impl<'de> Deserialize<'de> for Chat {
  fn deserialize<D>(deserializer: D) -> Result<Chat, D::Error>
    where D: Deserializer<'de>
  {
    let raw: RawChat = Deserialize::deserialize(deserializer)?;

    macro_rules! required_field {
      ($name:ident) => {{
        match raw.$name {
          Some(val) => val,
          None => return Err(D::Error::missing_field(stringify!($name)))
        }
      }}
    }

    Ok(match raw.type_.as_ref() {
      "private" => {
        Chat::Private(User {
          id: raw.id,
          username: raw.username,
          first_name: required_field!(first_name),
          last_name: raw.last_name,
          is_bot: raw.is_bot,
        })
      }
      "group" => {
        Chat::Group(Group {
          id: raw.id,
          title: required_field!(title),
          all_members_are_administrators: required_field!(all_members_are_administrators),
        })
      }
      "supergroup" => {
        Chat::Supergroup(Supergroup {
          id: raw.id,
          title: required_field!(title),
          username: raw.username,
        })
      }
      "channel" => {
        Chat::Channel(Channel {
          id: raw.id,
          title: required_field!(title),
          username: raw.username,
        })
      }
      _ => Chat::Unknown(raw),
    })
  }
}

/// This object represents a chat, directly mapped.
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Deserialize)]
pub struct RawChat {
  /// Unique identifier for this chat.
  pub id: i64,
  /// Type of chat, can be either “private”, “group”, “supergroup” or “channel”
  #[serde(rename = "type")]
  pub type_: String,
  /// Title, for supergroups, channels and group chats
  pub title: Option<String>,
  /// Username, for private chats, supergroups and channels if available
  pub username: Option<String>,
  /// First name of the other party in a private chat
  pub first_name: Option<String>,
  /// Last name of the other party in a private chat
  pub last_name: Option<String>,
  /// True if a group has ‘All Members Are Admins’ enabled.
  pub all_members_are_administrators: Option<bool>,
  /// Is bot
  pub is_bot: Option<bool>,
}
