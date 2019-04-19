use std::sync::Arc;

use rstring_builder::StringBuilder;
use text_reader::TextReader;

use crate::api::BotApi;
use crate::listener::Lout;
use crate::tglog;
use crate::types::{MessageEntityKind, RawMessage, MessageEntity};
use crate::vision::{Message, VCommand, VTextMessage};

pub fn handle_text(api: BotApi, lout: &Arc<Lout>, raw: &RawMessage, message: Message) {
  let text = raw.text.clone().unwrap();
  let entities = raw.entities.clone().unwrap_or_else(|| Vec::with_capacity(0));

  debug!(tglog::advanced(), "entities is empty | {:?}", entities.is_empty());
  debug!(tglog::advanced(), "entities => {:?}", entities);
  if entities.is_empty() {
    if let Some(fnc) = lout.listen_text() {
      let obj = VTextMessage { message, text, entities };
      (*fnc)((api, obj));
      return;
    }
    return;
  }

  let first = entities.get(0).unwrap();
  if first.kind == MessageEntityKind::BotCommand {
    handle_command(api, lout, message, &text, entities);
  }
}

fn handle_command(api: BotApi, lout: &Arc<Lout>, message: Message, text: &String, entities: Vec<MessageEntity>) {
  let (command, args) = extra_command(text);
  debug!(tglog::advanced(), "COMMAND: {:?} => ARGS: {:?}", command, args);

  let lcd = lout.listen_command();
  if let Some(fnc) = lcd.get(&command[..]) {
    let vcmd = VCommand {
      message,
      text: text.clone(),
      entities,
      command,
      args,
    };
    (*fnc)((api, vcmd));
  }
}


fn extra_command(text: &String) -> (String, Vec<String>) {
  let mut reader = TextReader::new(text);
  let mut command = "".to_string();
  let mut builder = StringBuilder::new();
  let mut args = Vec::new();

  let mut entry_command = false;
//  let mut check_command = false;
  let mut quote_ch = None;
  let mut entry_quote = false;
  while reader.has_next() {
    match reader.next() {
      Some('/') => {
        if reader.position() == 1 {
          entry_command = true;
          continue;
        }
        builder.append('/');
      }
      Some('\'') => {
        if entry_command {
          builder.append('\'');
          continue;
        }
        if entry_quote {
          if quote_ch == Some('\'') {
            quote_ch = None;
            entry_quote = false;
            continue;
          }
          builder.append('\'');
          continue;
        }
        quote_ch = Some('\'');
        entry_quote = true;
        continue;
      }
      Some('"') => {
        if entry_command {
          builder.append('"');
          continue;
        }
        if entry_quote {
          if quote_ch == Some('"') {
            quote_ch = None;
            entry_quote = false;
            continue;
          }
          builder.append('"');
          continue;
        }
        quote_ch = Some('"');
        entry_quote = true;
        continue;
      }
      Some(' ') => {
        if entry_quote {
          builder.append(' ');
          continue;
        }
        if entry_command {
          entry_command = false;
//          check_command = true;
          command = builder.string();
          builder.clear();
          continue;
        }
        if !entry_command {
          if builder.is_empty() {
            continue;
          }
          args.push(builder.string());
          builder.clear();
          continue;
        }
        continue;
      }
      Some(ch) => {
        builder.append(ch);
      }
      _ => {}
    }
  }

  if entry_command {
    command = builder.string();
  } else {
    args.push(builder.string());
  }
  builder.clear();


  if command.contains("@") {
    let mut reader = TextReader::new(command);
    while reader.has_next() {
      match reader.next() {
        Some('@') => {
          if reader.next() == Some('@') {
            builder.append('@');
            reader.back();
            continue;
          }
          break;
        },
        Some(ch) => builder.append(ch),
        None => continue,
      };
    }
    let command = builder.string();
    builder.clear();
    return (command, args);
  }

  (command, args)
}
