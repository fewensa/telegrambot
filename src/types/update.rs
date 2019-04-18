use std::fmt;

use serde::de::{Deserialize, Deserializer, Error, MapAccess, Visitor};
use serde_value::Value;

use crate::types::callback_query::CallbackQuery;
use crate::types::message::RawMessage;

/// This object represents an incoming update.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Update {
  /// The update‘s unique identifier. Update identifiers start from a certain
  /// positive number and increase sequentially.
//  #[serde(rename = "update_id")]
  pub id: i64,
  //  pub message: Option<RawMessage>,
//  pub callback_query: Option<CallbackQuery>,
  pub is_edited: bool,
  //  pub error: Option<String>,
  pub kind: UpdateKind,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum UpdateKind {
  Message(RawMessage),
  Channel(RawMessage),
  CallbackQuery(CallbackQuery),
  #[doc(hidden)]
  Err(String),
  #[doc(hidden)]
  Unknown,
}

///// This object represents an incoming update.
//#[derive(Debug, Clone, PartialEq, PartialOrd)]
//pub struct Update {
//  /// The update‘s unique identifier. Update identifiers start from a certain
//  /// positive number and increase sequentially.
//  pub id: i64,
//  /// Kind of the incoming update.
//  pub kind: UpdateKind,
//}
//
///// Kind of the incoming update.
//#[derive(Debug, Clone, PartialEq, PartialOrd)]
//pub enum UpdateKind {
//  /// New incoming message of any kind — text, photo, sticker, etc.
//  Message(Message),
//  /// New version of a message that is known to the bot and was edited
//  EditedMessage(Message),
//  /// New incoming channel post of any kind — text, photo, sticker, etc.
//  ChannelPost(ChannelPost),
//  /// New version of a channel post that is known to the bot and was edited
//  EditedChannelPost(ChannelPost),
//  // InlineQuery(InlineQuery),
//  // ChosenInlineResult(ChosenInlineResult),
//  CallbackQuery(CallbackQuery),
//  #[doc(hidden)]
//  Error(String),
//  #[doc(hidden)]
//  Unknown,
//}

impl<'de> Deserialize<'de> for Update {
  fn deserialize<D>(deserializer: D) -> Result<Update, D::Error> where D: Deserializer<'de> {
    #[derive(Deserialize)]
    #[serde(field_identifier, rename_all = "snake_case")]
    enum Field {
      UpdateId,
      Message,
      EditedMessage,
      ChannelPost,
      EditedChannelPost,
      CallbackQuery,
    }

    struct UpdateVisitor;

    impl<'de> Visitor<'de> for UpdateVisitor {
      type Value = Update;

      fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Update")
      }

      fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error> where V: MapAccess<'de> {
        let mut update_id = None;
        let mut error: Option<V::Error> = None;

        macro_rules! match_fields {
          ($(($variant:ident, $name:ident, $is_edit:ident, $kind:ident));*; ) => {{
            $(
              let mut $name = None;
            )*

            while let Some(key) = map.next_key()? {
              match key {
                Field::UpdateId => {
                  if update_id.is_some() {
                    return Err(Error::duplicate_field("update_id"));
                  }
                  update_id = Some(map.next_value()?)
                },
                $(
                  Field::$variant => {
                    if $name.is_some() {
                      return Err(Error::duplicate_field(stringify!($name)));
                    }
                    let value = map.next_value::<Value>()?;
                    match value.deserialize_into() {
                      Ok(value) => $name = Some(value),
                      Err(err) => error = Some(Error::custom(err)),
                    }
                  }
                )*
              }
            }

            let update_id = update_id.ok_or_else(|| Error::missing_field("update_id"))?;

            if let Some(err) = error {
              return Ok(Update {
                id: update_id,
                is_edited: false,
                kind: UpdateKind::Err(format!("{}", err))
              })
            }

            $(
              if let Some(val) = $name {
                return Ok(Update {
                  id: update_id,
                  is_edited: $is_edit,
                  kind: UpdateKind::$kind(val),
                })
              }
            )*

            return Ok(Update {
                id: update_id,
                is_edited: false,
                kind: UpdateKind::Unknown,
            })
          }};
        }

        match_fields!(
          (Message, message, false, Message);
          (EditedMessage, edited_message, true, Message);
          (ChannelPost, channel_post, false, Channel);
          (EditedChannelPost, edited_channel_post, true, Channel);
          (CallbackQuery, callback_query, false, CallbackQuery);
        )
      }
    }

    const FIELDS: &'static [&'static str] = &[
      "update_id",
      "message", "edited_message",
      "channel_post", "edited_channel_post",
      "callback_query",
    ];

    deserializer.deserialize_struct("Duration", FIELDS, UpdateVisitor)
  }
}
