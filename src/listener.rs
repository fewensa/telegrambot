use std::collections::HashMap;
use std::sync::Arc;

use crate::advanced::Track;
use crate::types::Update;
use crate::vision::*;
use crate::api::BotApi;

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum ListenerType {
  Text,
  Command,
  CallbackQuery,
  Update,
}

#[derive(Clone)]
pub struct Listener {
  text_handler: Option<Arc<Box<dyn Fn((&BotApi, &VTextMessage)) + Send + Sync + 'static>>>,
  update_handler: Option<Arc<Box<dyn Fn((&BotApi, &Update)) + Send + Sync + 'static>>>,
  audio_handler: Option<Arc<Box<dyn Fn((&BotApi, &VAudioMessage)) + Send + Sync + 'static>>>,
  document_handler: Option<Arc<Box<dyn Fn((&BotApi, &VDocumentMessage)) + Send + Sync + 'static>>>,
  photo_handler: Option<Arc<Box<dyn Fn((&BotApi, &VPhotoMessage)) + Send + Sync + 'static>>>,
  sticker_handler: Option<Arc<Box<dyn Fn((&BotApi, &VStickerMessage)) + Send + Sync + 'static>>>,
  video_handler: Option<Arc<Box<dyn Fn((&BotApi, &VVideoMessage)) + Send + Sync + 'static>>>,
  voice_handler: Option<Arc<Box<dyn Fn((&BotApi, &VVoiceMessage)) + Send + Sync + 'static>>>,
  video_note_handler: Option<Arc<Box<dyn Fn((&BotApi, &VVideoNoteMessage)) + Send + Sync + 'static>>>,
  contact_handler: Option<Arc<Box<dyn Fn((&BotApi, &VContactMessage)) + Send + Sync + 'static>>>,
  location_handler: Option<Arc<Box<dyn Fn((&BotApi, &VLocationMessage)) + Send + Sync + 'static>>>,
  venue_handler: Option<Arc<Box<dyn Fn((&BotApi, &VVenueMessage)) + Send + Sync + 'static>>>,
  new_chat_members_handler: Option<Arc<Box<dyn Fn((&BotApi, &VNewChatMembersMessage)) + Send + Sync + 'static>>>,
  left_chat_member_handler: Option<Arc<Box<dyn Fn((&BotApi, &VLeftChatMemberMessage)) + Send + Sync + 'static>>>,
  chat_title_handler: Option<Arc<Box<dyn Fn((&BotApi, &VChatTitleMessage)) + Send + Sync + 'static>>>,
  chat_photo_handler: Option<Arc<Box<dyn Fn((&BotApi, &VChatPhotoMessage)) + Send + Sync + 'static>>>,
  delete_chat_photo_handler: Option<Arc<Box<dyn Fn((&BotApi, &Message)) + Send + Sync + 'static>>>,
  group_chat_created_handler: Option<Arc<Box<dyn Fn((&BotApi, &Message)) + Send + Sync + 'static>>>,
  supergroup_chat_created_handler: Option<Arc<Box<dyn Fn((&BotApi, &Message)) + Send + Sync + 'static>>>,
  channel_chat_created_handler: Option<Arc<Box<dyn Fn((&BotApi, &Message)) + Send + Sync + 'static>>>,
  migrate_to_chat_id_handler: Option<Arc<Box<dyn Fn((&BotApi, &VMigrateToChatIdMessage)) + Send + Sync + 'static>>>,
  migrate_from_chat_id_handler: Option<Arc<Box<dyn Fn((&BotApi, &VMigrateFromChatIdMessage)) + Send + Sync + 'static>>>,
  pinned_message_handler: Option<Arc<Box<dyn Fn((&BotApi, &VPinnedMessageMessage)) + Send + Sync + 'static>>>,

  callback_query_handler: Option<Arc<Box<dyn Fn((&BotApi, &VCallbackQuery)) + Send + Sync + 'static>>>,
  error_handler: Option<Arc<Box<dyn Fn((&BotApi, &String)) + Send + Sync + 'static>>>,

  command_handler: HashMap<&'static str, Arc<Box<dyn Fn((&BotApi, &VCommand)) + Send + Sync + 'static>>>,
}

impl Default for Listener {
  fn default() -> Self {
    Listener {
      text_handler: None,
      update_handler: None,
      audio_handler: None,
      document_handler: None,
      photo_handler: None,
      sticker_handler: None,
      video_handler: None,
      voice_handler: None,
      video_note_handler: None,
      contact_handler: None,
      location_handler: None,
      venue_handler: None,
      new_chat_members_handler: None,
      left_chat_member_handler: None,
      chat_title_handler: None,
      chat_photo_handler: None,
      delete_chat_photo_handler: None,
      group_chat_created_handler: None,
      supergroup_chat_created_handler: None,
      channel_chat_created_handler: None,
      migrate_to_chat_id_handler: None,
      migrate_from_chat_id_handler: None,
      pinned_message_handler: None,
      callback_query_handler: None,
      error_handler: None,
      command_handler: HashMap::new(),
    }
  }
}


impl Listener {
  pub fn on_update<F>(&mut self, fnc: F) -> &mut Self where F: Fn((&BotApi, &Update)) + Send + Sync + 'static {
    self.update_handler = Some(Arc::new(Box::new(fnc)));
    self
  }

  pub fn on_callback_query<F>(&mut self, fnc: F) -> &mut Self where F: Fn((&BotApi, &VCallbackQuery)) + Send + Sync + 'static {
    self.callback_query_handler = Some(Arc::new(Box::new(fnc)));
    self
  }

  pub fn on_error<F>(&mut self, fnc: F) -> &mut Self where F: Fn((&BotApi, &String)) + Send + Sync + 'static {
    self.error_handler = Some(Arc::new(Box::new(fnc)));
    self
  }


  pub fn on_command<S, F>(&mut self, command: S, fnc: F) -> &mut Self where
    S: AsRef<str> + 'static,
    F: Fn((&BotApi, &VCommand)) + Send + Sync + 'static {
    let mut command = command.as_ref();
    if command.starts_with("/") {
      command = &command[1..];
    }
    let command: &'static str = Box::leak(command.to_string().into_boxed_str());
    self.command_handler.insert(command, Arc::new(Box::new(fnc)));
    self
  }

  pub fn on_text<F>(&mut self, fnc: F) -> &mut Self where F: Fn((&BotApi, &VTextMessage)) + Send + Sync + 'static {
    self.text_handler = Some(Arc::new(Box::new(fnc)));
    self
  }

  pub fn on_audio<F>(&mut self, fnc: F) -> &mut Self where F: Fn((&BotApi, &VAudioMessage)) + Send + Sync + 'static {
    self.audio_handler = Some(Arc::new(Box::new(fnc)));
    self
  }


  pub fn on_document<F>(&mut self, fnc: F) -> &mut Self where F: Fn((&BotApi, &VDocumentMessage)) + Send + Sync + 'static {
    self.document_handler = Some(Arc::new(Box::new(fnc)));
    self
  }

  pub fn on_photo<F>(&mut self, fnc: F) -> &mut Self where F: Fn((&BotApi, &VPhotoMessage)) + Send + Sync + 'static {
    self.photo_handler = Some(Arc::new(Box::new(fnc)));
    self
  }

  pub fn on_sticker<F>(&mut self, fnc: F) -> &mut Self where F: Fn((&BotApi, &VStickerMessage)) + Send + Sync + 'static {
    self.sticker_handler = Some(Arc::new(Box::new(fnc)));
    self
  }

  pub fn on_video<F>(&mut self, fnc: F) -> &mut Self where F: Fn((&BotApi, &VVideoMessage)) + Send + Sync + 'static {
    self.video_handler = Some(Arc::new(Box::new(fnc)));
    self
  }

  pub fn on_voice<F>(&mut self, fnc: F) -> &mut Self where F: Fn((&BotApi, &VVoiceMessage)) + Send + Sync + 'static {
    self.voice_handler = Some(Arc::new(Box::new(fnc)));
    self
  }

  pub fn on_video_note<F>(&mut self, fnc: F) -> &mut Self where F: Fn((&BotApi, &VVideoNoteMessage)) + Send + Sync + 'static {
    self.video_note_handler = Some(Arc::new(Box::new(fnc)));
    self
  }

  pub fn on_contact<F>(&mut self, fnc: F) -> &mut Self where F: Fn((&BotApi, &VContactMessage)) + Send + Sync + 'static {
    self.contact_handler = Some(Arc::new(Box::new(fnc)));
    self
  }

  pub fn on_location<F>(&mut self, fnc: F) -> &mut Self where F: Fn((&BotApi, &VLocationMessage)) + Send + Sync + 'static {
    self.location_handler = Some(Arc::new(Box::new(fnc)));
    self
  }

  pub fn on_venue<F>(&mut self, fnc: F) -> &mut Self where F: Fn((&BotApi, &VVenueMessage)) + Send + Sync + 'static {
    self.venue_handler = Some(Arc::new(Box::new(fnc)));
    self
  }

  pub fn on_new_chat_members<F>(&mut self, fnc: F) -> &mut Self where F: Fn((&BotApi, &VNewChatMembersMessage)) + Send + Sync + 'static {
    self.new_chat_members_handler = Some(Arc::new(Box::new(fnc)));
    self
  }

  pub fn on_left_chat_member<F>(&mut self, fnc: F) -> &mut Self where F: Fn((&BotApi, &VLeftChatMemberMessage)) + Send + Sync + 'static {
    self.left_chat_member_handler = Some(Arc::new(Box::new(fnc)));
    self
  }

  pub fn on_new_chat_title<F>(&mut self, fnc: F) -> &mut Self where F: Fn((&BotApi, &VChatTitleMessage)) + Send + Sync + 'static {
    self.chat_title_handler = Some(Arc::new(Box::new(fnc)));
    self
  }

  pub fn on_new_chat_photo<F>(&mut self, fnc: F) -> &mut Self where F: Fn((&BotApi, &VChatPhotoMessage)) + Send + Sync + 'static {
    self.chat_photo_handler = Some(Arc::new(Box::new(fnc)));
    self
  }

  pub fn on_delete_chat_photo<F>(&mut self, fnc: F) -> &mut Self where F: Fn((&BotApi, &Message)) + Send + Sync + 'static {
    self.delete_chat_photo_handler = Some(Arc::new(Box::new(fnc)));
    self
  }

  pub fn on_group_chat_created<F>(&mut self, fnc: F) -> &mut Self where F: Fn((&BotApi, &Message)) + Send + Sync + 'static {
    self.group_chat_created_handler = Some(Arc::new(Box::new(fnc)));
    self
  }

  pub fn on_supergroup_chat_created<F>(&mut self, fnc: F) -> &mut Self where F: Fn((&BotApi, &Message)) + Send + Sync + 'static {
    self.supergroup_chat_created_handler = Some(Arc::new(Box::new(fnc)));
    self
  }

  pub fn on_channel_chat_create<F>(&mut self, fnc: F) -> &mut Self where F: Fn((&BotApi, &Message)) + Send + Sync + 'static {
    self.channel_chat_created_handler = Some(Arc::new(Box::new(fnc)));
    self
  }

  pub fn on_migrate_to_chat<F>(&mut self, fnc: F) -> &mut Self where F: Fn((&BotApi, &VMigrateToChatIdMessage)) + Send + Sync + 'static {
    self.migrate_to_chat_id_handler = Some(Arc::new(Box::new(fnc)));
    self
  }

  pub fn on_migrate_from_chat<F>(&mut self, fnc: F) -> &mut Self where F: Fn((&BotApi, &VMigrateFromChatIdMessage)) + Send + Sync + 'static {
    self.migrate_from_chat_id_handler = Some(Arc::new(Box::new(fnc)));
    self
  }

  pub fn on_pinned<F>(&mut self, fnc: F) -> &mut Self where F: Fn((&BotApi, &VPinnedMessageMessage)) + Send + Sync + 'static {
    self.pinned_message_handler = Some(Arc::new(Box::new(fnc)));
    self
  }
}

pub struct Lout {
  listener: Listener
}

impl Lout {
  pub fn new(listener: Listener) -> Self {
    Lout { listener }
  }

  pub fn listen_command(&self) -> &HashMap<&'static str, Arc<Box<dyn Fn((&BotApi, &VCommand)) + Send + Sync + 'static>>> {
    &self.listener.command_handler
  }

  pub fn listen_error(&self) -> &Option<Arc<Box<dyn Fn((&BotApi, &String)) + Send + Sync + 'static>>> {
    &self.listener.error_handler
  }

  pub fn listen_callback_query(&self) -> &Option<Arc<Box<dyn Fn((&BotApi, &VCallbackQuery)) + Send + Sync + 'static>>> {
    &self.listener.callback_query_handler
  }

  pub fn listen_update(&self) -> &Option<Arc<Box<dyn Fn((&BotApi, &Update)) + Send + Sync + 'static>>> {
    &self.listener.update_handler
  }

  pub fn listen_text(&self) -> &Option<Arc<Box<dyn Fn((&BotApi, &VTextMessage)) + Send + Sync + 'static>>> {
    &self.listener.text_handler
  }

  pub fn listen_audio(&self) -> &Option<Arc<Box<dyn Fn((&BotApi, &VAudioMessage)) + Send + Sync + 'static>>> {
    &self.listener.audio_handler
  }

  pub fn listen_document(&self) -> &Option<Arc<Box<dyn Fn((&BotApi, &VDocumentMessage)) + Send + Sync + 'static>>> {
    &self.listener.document_handler
  }

  pub fn listen_photo(&self) -> &Option<Arc<Box<dyn Fn((&BotApi, &VPhotoMessage)) + Send + Sync + 'static>>> {
    &self.listener.photo_handler
  }

  pub fn listen_sticker(&self) -> &Option<Arc<Box<dyn Fn((&BotApi, &VStickerMessage)) + Send + Sync + 'static>>> {
    &self.listener.sticker_handler
  }

  pub fn listen_video(&self) -> &Option<Arc<Box<dyn Fn((&BotApi, &VVideoMessage)) + Send + Sync + 'static>>> {
    &self.listener.video_handler
  }

  pub fn listen_voice(&self) -> &Option<Arc<Box<dyn Fn((&BotApi, &VVoiceMessage)) + Send + Sync + 'static>>> {
    &self.listener.voice_handler
  }

  pub fn listen_video_note(&self) -> &Option<Arc<Box<dyn Fn((&BotApi, &VVideoNoteMessage)) + Send + Sync + 'static>>> {
    &self.listener.video_note_handler
  }

  pub fn listen_contact(&self) -> &Option<Arc<Box<dyn Fn((&BotApi, &VContactMessage)) + Send + Sync + 'static>>> {
    &self.listener.contact_handler
  }

  pub fn listen_location(&self) -> &Option<Arc<Box<dyn Fn((&BotApi, &VLocationMessage)) + Send + Sync + 'static>>> {
    &self.listener.location_handler
  }

  pub fn listen_venue(&self) -> &Option<Arc<Box<dyn Fn((&BotApi, &VVenueMessage)) + Send + Sync + 'static>>> {
    &self.listener.venue_handler
  }

  pub fn listen_new_chat_members(&self) -> &Option<Arc<Box<dyn Fn((&BotApi, &VNewChatMembersMessage)) + Send + Sync + 'static>>> {
    &self.listener.new_chat_members_handler
  }

  pub fn listen_left_chat_member(&self) -> &Option<Arc<Box<dyn Fn((&BotApi, &VLeftChatMemberMessage)) + Send + Sync + 'static>>> {
    &self.listener.left_chat_member_handler
  }

  pub fn listen_new_chat_title(&self) -> &Option<Arc<Box<dyn Fn((&BotApi, &VChatTitleMessage)) + Send + Sync + 'static>>> {
    &self.listener.chat_title_handler
  }

  pub fn listen_new_chat_photo(&self) -> &Option<Arc<Box<dyn Fn((&BotApi, &VChatPhotoMessage)) + Send + Sync + 'static>>> {
    &self.listener.chat_photo_handler
  }

  pub fn listen_delete_chat_photo(&self) -> &Option<Arc<Box<dyn Fn((&BotApi, &Message)) + Send + Sync + 'static>>> {
    &self.listener.delete_chat_photo_handler
  }

  pub fn listen_group_chat_created(&self) -> &Option<Arc<Box<dyn Fn((&BotApi, &Message)) + Send + Sync + 'static>>> {
    &self.listener.group_chat_created_handler
  }

  pub fn listen_supergroup_chat_created(&self) -> &Option<Arc<Box<dyn Fn((&BotApi, &Message)) + Send + Sync + 'static>>> {
    &self.listener.supergroup_chat_created_handler
  }

  pub fn listen_channel_chat_create(&self) -> &Option<Arc<Box<dyn Fn((&BotApi, &Message)) + Send + Sync + 'static>>> {
    &self.listener.channel_chat_created_handler
  }

  pub fn listen_migrate_to_chat(&self) -> &Option<Arc<Box<dyn Fn((&BotApi, &VMigrateToChatIdMessage)) + Send + Sync + 'static>>> {
    &self.listener.migrate_to_chat_id_handler
  }

  pub fn listen_migrate_from_chat(&self) -> &Option<Arc<Box<dyn Fn((&BotApi, &VMigrateFromChatIdMessage)) + Send + Sync + 'static>>> {
    &self.listener.migrate_from_chat_id_handler
  }

  pub fn listen_pinned(&self) -> &Option<Arc<Box<dyn Fn((&BotApi, &VPinnedMessageMessage)) + Send + Sync + 'static>>> {
    &self.listener.pinned_message_handler
  }
}
