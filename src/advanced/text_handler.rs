use std::sync::Arc;

use crate::tglog;
use crate::advanced::Track;
use crate::listener::Lout;
use crate::types::{ChannelPost, Message, MessageEntity, MessageKind};
use crate::vision::TextMessage;

//#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Hash)]
//enum MessageEntityKind {
//  Mention,
//  Hashtag,
//  BotCommand,
//  Url,
//  Email,
//  Bold,
//  Italic,
//  Code,
//  Pre,
//  TextLink,
//  // TODO(knsd) URL?
//  TextMention,
//  #[doc(hidden)]
//  Unknown,
//}


pub fn handle_message(update_id: i64, edited: bool,
                      message: &Message, data: &String, entities: &Vec<MessageEntity>,
                      lout: &Arc<Lout>) {
  debug!(tglog::advanced(), "DATA: {:?} ENTITIES: {:?}", data, entities);

  let listen_text = lout.listen_text(Track::Message);
  if let None = listen_text {
    return;
  }
  let listen_text = listen_text.unwrap();

  let atm = TextMessage {
    id: message.id.clone(),
    from: message.from.clone(),
    date: message.date,
    chat: message.chat.clone(),
    forward: message.forward.clone(),
    reply_to_message: message.reply_to_message.clone(),
    edit_date: message.edit_date,
    text: data.clone(),
    entities: entities.clone()
  };
  debug!(tglog::advanced(), "TextMessage: {:?}", atm);
  (*listen_text)((&atm, edited));
}

pub fn handle_channel_post(update_id: i64, edited: bool,
                           message: &ChannelPost, data: &String, entities: &Vec<MessageEntity>,
                           lout: &Arc<Lout>) {

}


