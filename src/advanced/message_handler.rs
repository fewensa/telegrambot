use std::sync::Arc;

use crate::listener::Lout;
use crate::tglog;
use crate::types::*;
use crate::vision::*;
use crate::advanced::text_handler;
use crate::api::BotApi;

pub fn handle(api: &BotApi, lout: &Arc<Lout>, raw: &RawMessage, is_edited: bool) {
  let message = to_message(raw);


  macro_rules! maybe_field {
    ($name:ident, $variant:ident, $field:ident, $fnc:ident) => {{
      if let Some(val) = &raw.$name {
        let obj = $variant {
          message,
          $field: val.clone(),
        };
        if let Some(fnc) = lout.$fnc() {
          (*fnc)((api, &obj));
        }
        return;
      }
    }}
  }

  macro_rules! maybe_field_with_caption {
    ($name:ident, $variant:ident, $field:ident, $fnc:ident) => {{
      if let Some(fnc) = lout.$fnc() {
        if let Some(val) = &raw.$name {
          let obj = $variant {
            message,
            $field: val.clone(),
            caption: raw.caption.clone(),
          };
          (*fnc)((api, &obj));
          return;
        }
      }
    }}
  }

  macro_rules! maybe_field_with_caption_and_group {
    ($name:ident, $variant:ident, $field:ident, $fnc:ident) => {{
      if let Some(fnc) = lout.$fnc() {
        if let Some(val) = &raw.$name {
          let obj = $variant {
            message,
            $field: val.clone(),
            caption: raw.caption.clone(),
            media_group_id: raw.media_group_id.clone()
          };
          (*fnc)((api, &obj));
          return;
        }
      }
    }}
  }

  macro_rules! maybe_true_field {
    ($name:ident, $fnc:ident) => {{
      if let Some(fnc) = lout.$fnc() {
        if let Some(True) = &raw.$name {
          (*fnc)((api, &message));
          return;
        }
      }
    }}
  }

  if let Some(_) = &raw.text {
    text_handler::handle_text(api, lout, raw, message);
    return;
  }


  maybe_field!                       ( audio,                   VAudioMessage,                   audio,                 listen_audio                    );
  maybe_field_with_caption!          ( document,                VDocumentMessage,                document,              listen_document                 );
  maybe_field_with_caption_and_group!( photo,                   VPhotoMessage,                   photo,                 listen_photo                    );
  maybe_field!                       ( sticker,                 VStickerMessage,                 sticker,               listen_sticker                  );
  maybe_field_with_caption_and_group!( video,                   VVideoMessage,                   video,                 listen_video                    );
  maybe_field!                       ( voice,                   VVoiceMessage,                   voice,                 listen_voice                    );
  maybe_field!                       ( video_note,              VVideoNoteMessage,               video_note,            listen_video_note               );
  maybe_field!                       ( contact,                 VContactMessage,                 contact,               listen_contact                  );
  maybe_field!                       ( location,                VLocationMessage,                location,              listen_location                 );
  maybe_field!                       ( venue,                   VVenueMessage,                   venue,                 listen_venue                    );
  maybe_field!                       ( new_chat_members,        VNewChatMembersMessage,          members,               listen_new_chat_members         );
  maybe_field!                       ( left_chat_member,        VLeftChatMemberMessage,          member,                listen_left_chat_member         );
  maybe_field!                       ( new_chat_title,          VChatTitleMessage,               title,                 listen_new_chat_title           );
  maybe_field!                       ( new_chat_photo,          VChatPhotoMessage,               photos,                listen_new_chat_photo           );
  maybe_true_field!                  ( delete_chat_photo,                                                               listen_delete_chat_photo        );
  maybe_true_field!                  ( group_chat_created,                                                              listen_group_chat_created       );
  maybe_true_field!                  ( supergroup_chat_created,                                                         listen_supergroup_chat_created  );
  maybe_true_field!                  ( channel_chat_created,                                                            listen_channel_chat_create      );
  maybe_field!                       ( migrate_to_chat_id,      VMigrateToChatIdMessage,         migrate_to_chat_id,    listen_migrate_to_chat          );
  maybe_field!                       ( migrate_from_chat_id,    VMigrateFromChatIdMessage,       migrate_from_chat_id,  listen_migrate_from_chat        );
  maybe_field!                       ( pinned_message,          VPinnedMessageMessage,           pinned,                listen_pinned                   );
//    make_message(MessageKind::Unknown { raw: raw })
}


pub fn to_message(raw: &RawMessage) -> Message {
  Message {
    id: raw.message_id,
    from: raw.from.clone(),
    date: raw.date,
//    chat: raw.chat.clone(),
    forward: gen_forward(&raw),
    reply_to_message: raw.reply_to_message.clone().map(|raw| PossibilityMessage::new(*raw)),
    edit_date: raw.edit_date,
    chat: gen_chat(raw),
  }
}

fn gen_chat(raw: &RawMessage) -> VMessagChat {
  match raw.chat {
    Chat::Channel(ref channel) => VMessagChat::Channel(channel.clone()),
    Chat::Private(ref user) => VMessagChat::Message(MessageChat::Private(user.clone())),
    Chat::Group(ref group) => VMessagChat::Message(MessageChat::Group(group.clone())),
    Chat::Supergroup(ref supergroup) => VMessagChat::Message(MessageChat::Supergroup(supergroup.clone())),
    Chat::Unknown(ref rawchat) => VMessagChat::Message(MessageChat::Unknown(rawchat.clone()))
  }
}

fn gen_forward(raw: &RawMessage) -> Option<Forward> {
  match (raw.forward_date,
         &raw.forward_from,
         &raw.forward_from_chat,
         raw.forward_from_message_id,
         &raw.forward_sender_name) {
    (None, &None, &None, None, &None) => None,
    (Some(date), &Some(ref from), &None, None, &None) => {
      Some(Forward {
        date,
        from: ForwardFrom::User { user: from.clone() },
      })
    }
    (Some(date), &None, &Some(Chat::Channel(ref channel)), Some(message_id), &None) => {
      Some(Forward {
        date,
        from: ForwardFrom::Channel {
          channel: channel.clone(),
          message_id,
        },
      })
    }
    (Some(date), &None, &None, None, &Some(ref sender_name)) => {
      Some(Forward {
        date,
        from: ForwardFrom::ChannelHiddenUser { sender_name: sender_name.clone() },
      })
    }
    _ => None,
  }
}




