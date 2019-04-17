use std::sync::Arc;

use crate::advanced::Track;
use crate::types::{Update};
use crate::vision::*;

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum ListenerType {
  Text,
  Command,
  CallbackQuery,
  Update,
}

#[derive(Clone)]
pub struct Listener {
  text_handler: Option<Arc<Box<dyn Fn(&VTextMessage) + Send + Sync + 'static>>>,
  update_handler: Option<Arc<Box<dyn Fn(&Update) + Send + Sync + 'static>>>,
  audio_handler: Option<Arc<Box<dyn Fn(&VAudioMessage) + Send + Sync + 'static>>>,
  document_handler: Option<Arc<Box<dyn Fn(&VDocumentMessage) + Send + Sync + 'static>>>,
  photo_handler: Option<Arc<Box<dyn Fn(&VPhotoMessage) + Send + Sync + 'static>>>,
  sticker_handler: Option<Arc<Box<dyn Fn(&VStickerMessage) + Send + Sync + 'static>>>,
  video_handler: Option<Arc<Box<dyn Fn(&VVideoMessage) + Send + Sync + 'static>>>,
  voice_handler: Option<Arc<Box<dyn Fn(&VVoiceMessage) + Send + Sync + 'static>>>,
  video_note_handler: Option<Arc<Box<dyn Fn(&VVideoNoteMessage) + Send + Sync + 'static>>>,
  contact_handler: Option<Arc<Box<dyn Fn(&VContactMessage) + Send + Sync + 'static>>>,
  location_handler: Option<Arc<Box<dyn Fn(&VLocationMessage) + Send + Sync + 'static>>>,
  venue_handler: Option<Arc<Box<dyn Fn(&VVenueMessage) + Send + Sync + 'static>>>,
  new_chat_members_handler: Option<Arc<Box<dyn Fn(&VNewChatMembersMessage) + Send + Sync + 'static>>>,
  left_chat_member_handler: Option<Arc<Box<dyn Fn(&VLeftChatMemberMessage) + Send + Sync + 'static>>>,
  chat_title_handler: Option<Arc<Box<dyn Fn(&VChatTitleMessage) + Send + Sync + 'static>>>,
  chat_photo_handler: Option<Arc<Box<dyn Fn(&VChatPhotoMessage) + Send + Sync + 'static>>>,
  delete_chat_photo_handler: Option<Arc<Box<dyn Fn(&Message) + Send + Sync + 'static>>>,
  group_chat_created_handler: Option<Arc<Box<dyn Fn(&Message) + Send + Sync + 'static>>>,
  supergroup_chat_created_handler: Option<Arc<Box<dyn Fn(&Message) + Send + Sync + 'static>>>,
  channel_chat_created_handler: Option<Arc<Box<dyn Fn(&Message) + Send + Sync + 'static>>>,
  migrate_to_chat_id_handler: Option<Arc<Box<dyn Fn(&VMigrateToChatIdMessage) + Send + Sync + 'static>>>,
  migrate_from_chat_id_handler: Option<Arc<Box<dyn Fn(&VMigrateFromChatIdMessage) + Send + Sync + 'static>>>,
  pinned_message_handler: Option<Arc<Box<dyn Fn(&VPinnedMessageMessage) + Send + Sync + 'static>>>,
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
    }
  }
}


impl Listener {
  pub fn on_update<F>(&mut self, fnc: F) -> &mut Self where F: Fn(&Update) + Send + Sync + 'static {
    self.update_handler = Some(Arc::new(Box::new(fnc)));
    self
  }

  pub fn on_callback_query(&mut self) -> &mut Self {
    self
  }

  pub fn on_command(&mut self, track: Track) -> &mut Self {
    self
  }

  pub fn on_text<F>(&mut self, fnc: F) -> &mut Self where F: Fn(&VTextMessage) + Send + Sync + 'static {
    self.text_handler = Some(Arc::new(Box::new(fnc)));
    self
  }

  pub fn on_audio<F>(&mut self, fnc: F) -> &mut Self where F: Fn(&VAudioMessage) + Send + Sync + 'static {
    self.audio_handler = Some(Arc::new(Box::new(fnc)));
    self
  }






  pub fn on_document<F>(&mut self, fnc: F) -> &mut Self where F: Fn(&VDocumentMessage) + Send + Sync + 'static {
    self.document_handler = Some(Arc::new(Box::new(fnc)));
    self
  }

  pub fn on_photo<F>(&mut self, fnc: F) -> &mut Self where F: Fn(&VPhotoMessage) + Send + Sync + 'static {
    self.photo_handler = Some(Arc::new(Box::new(fnc)));
    self
  }

  pub fn on_sticker<F>(&mut self, fnc: F) -> &mut Self where F: Fn(&VStickerMessage) + Send + Sync + 'static {
    self.sticker_handler = Some(Arc::new(Box::new(fnc)));
    self
  }

  pub fn on_video<F>(&mut self, fnc: F) -> &mut Self where F: Fn(&VVideoMessage) + Send + Sync + 'static {
    self.video_handler = Some(Arc::new(Box::new(fnc)));
    self
  }

  pub fn on_voice<F>(&mut self, fnc: F) -> &mut Self where F: Fn(&VVoiceMessage) + Send + Sync + 'static {
    self.voice_handler = Some(Arc::new(Box::new(fnc)));
    self
  }

  pub fn on_video_note<F>(&mut self, fnc: F) -> &mut Self where F: Fn(&VVideoNoteMessage) + Send + Sync + 'static {
    self.video_note_handler = Some(Arc::new(Box::new(fnc)));
    self
  }

  pub fn on_contact<F>(&mut self, fnc: F) -> &mut Self where F: Fn(&VContactMessage) + Send + Sync + 'static {
    self.contact_handler = Some(Arc::new(Box::new(fnc)));
    self
  }

  pub fn on_location<F>(&mut self, fnc: F) -> &mut Self where F: Fn(&VLocationMessage) + Send + Sync + 'static {
    self.location_handler = Some(Arc::new(Box::new(fnc)));
    self
  }

  pub fn on_venue<F>(&mut self, fnc: F) -> &mut Self where F: Fn(&VVenueMessage) + Send + Sync + 'static {
    self.venue_handler = Some(Arc::new(Box::new(fnc)));
    self
  }

  pub fn on_new_chat_members<F>(&mut self, fnc: F) -> &mut Self where F: Fn(&VNewChatMembersMessage) + Send + Sync + 'static {
    self.new_chat_members_handler = Some(Arc::new(Box::new(fnc)));
    self
  }

  pub fn on_left_chat_member<F>(&mut self, fnc: F) -> &mut Self where F: Fn(&VLeftChatMemberMessage) + Send + Sync + 'static {
    self.left_chat_member_handler = Some(Arc::new(Box::new(fnc)));
    self
  }

  pub fn on_new_chat_title<F>(&mut self, fnc: F) -> &mut Self where F: Fn(&VChatTitleMessage) + Send + Sync + 'static {
    self.chat_title_handler = Some(Arc::new(Box::new(fnc)));
    self
  }

  pub fn on_new_chat_photo<F>(&mut self, fnc: F) -> &mut Self where F: Fn(&VChatPhotoMessage) + Send + Sync + 'static {
    self.chat_photo_handler = Some(Arc::new(Box::new(fnc)));
    self
  }

  pub fn on_delete_chat_photo<F>(&mut self, fnc: F) -> &mut Self where F: Fn(&Message) + Send + Sync + 'static {
    self.delete_chat_photo_handler = Some(Arc::new(Box::new(fnc)));
    self
  }

  pub fn on_group_chat_created<F>(&mut self, fnc: F) -> &mut Self where F: Fn(&Message) + Send + Sync + 'static {
    self.group_chat_created_handler = Some(Arc::new(Box::new(fnc)));
    self
  }

  pub fn on_supergroup_chat_created<F>(&mut self, fnc: F) -> &mut Self where F: Fn(&Message) + Send + Sync + 'static {
    self.supergroup_chat_created_handler = Some(Arc::new(Box::new(fnc)));
    self
  }

  pub fn on_channel_chat_create<F>(&mut self, fnc: F) -> &mut Self where F: Fn(&Message) + Send + Sync + 'static {
    self.channel_chat_created_handler = Some(Arc::new(Box::new(fnc)));
    self
  }

  pub fn on_migrate_to_chat<F>(&mut self, fnc: F) -> &mut Self where F: Fn(&VMigrateToChatIdMessage) + Send + Sync + 'static {
    self.migrate_to_chat_id_handler = Some(Arc::new(Box::new(fnc)));
    self
  }

  pub fn on_migrate_from_chat<F>(&mut self, fnc: F) -> &mut Self where F: Fn(&VMigrateFromChatIdMessage) + Send + Sync + 'static {
    self.migrate_from_chat_id_handler = Some(Arc::new(Box::new(fnc)));
    self
  }

  pub fn on_pinned<F>(&mut self, fnc: F) -> &mut Self where F: Fn(&VPinnedMessageMessage) + Send + Sync + 'static {
    self.pinned_message_handler = Some(Arc::new(Box::new(fnc)));
    self
  }



  pub fn on_error(&mut self) -> &mut Self {
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

  pub fn listen_update(&self) -> &Option<Arc<Box<dyn Fn(&Update) + Send + Sync + 'static>>> {
    &self.listener.update_handler
  }

  pub fn listen_text(&self) -> &Option<Arc<Box<dyn Fn(&VTextMessage) + Send + Sync + 'static>>> {
    &self.listener.text_handler
  }

  pub fn listen_audio(&self) -> &Option<Arc<Box<dyn Fn(&VAudioMessage) + Send + Sync + 'static>>> {
    &self.listener.audio_handler
  }

  pub fn listen_document(&self) -> &Option<Arc<Box<dyn Fn(&VDocumentMessage) + Send + Sync + 'static>>> {
    &self.listener.document_handler
  }

  pub fn listen_photo(&self) -> &Option<Arc<Box<dyn Fn(&VPhotoMessage) + Send + Sync + 'static>>> {
    &self.listener.photo_handler
  }

  pub fn listen_sticker(&self) -> &Option<Arc<Box<dyn Fn(&VStickerMessage) + Send + Sync + 'static>>> {
    &self.listener.sticker_handler
  }

  pub fn listen_video(&self) -> &Option<Arc<Box<dyn Fn(&VVideoMessage) + Send + Sync + 'static>>> {
    &self.listener.video_handler
  }

  pub fn listen_voice(&self) -> &Option<Arc<Box<dyn Fn(&VVoiceMessage) + Send + Sync + 'static>>> {
    &self.listener.voice_handler
  }

  pub fn listen_video_note(&self) -> &Option<Arc<Box<dyn Fn(&VVideoNoteMessage) + Send + Sync + 'static>>> {
    &self.listener.video_note_handler
  }

  pub fn listen_contact(&self) -> &Option<Arc<Box<dyn Fn(&VContactMessage) + Send + Sync + 'static>>> {
    &self.listener.contact_handler
  }

  pub fn listen_location(&self) -> &Option<Arc<Box<dyn Fn(&VLocationMessage) + Send + Sync + 'static>>> {
    &self.listener.location_handler
  }

  pub fn listen_venue(&self) -> &Option<Arc<Box<dyn Fn(&VVenueMessage) + Send + Sync + 'static>>> {
    &self.listener.venue_handler
  }

  pub fn listen_new_chat_members(&self) -> &Option<Arc<Box<dyn Fn(&VNewChatMembersMessage) + Send + Sync + 'static>>> {
    &self.listener.new_chat_members_handler
  }

  pub fn listen_left_chat_member(&self) -> &Option<Arc<Box<dyn Fn(&VLeftChatMemberMessage) + Send + Sync + 'static>>> {
    &self.listener.left_chat_member_handler
  }

  pub fn listen_new_chat_title(&self) -> &Option<Arc<Box<dyn Fn(&VChatTitleMessage) + Send + Sync + 'static>>> {
    &self.listener.chat_title_handler
  }

  pub fn listen_new_chat_photo(&self) -> &Option<Arc<Box<dyn Fn(&VChatPhotoMessage) + Send + Sync + 'static>>> {
    &self.listener.chat_photo_handler
  }

  pub fn listen_delete_chat_photo(&self) -> &Option<Arc<Box<dyn Fn(&Message) + Send + Sync + 'static>>> {
    &self.listener.delete_chat_photo_handler
  }

  pub fn listen_group_chat_created(&self) -> &Option<Arc<Box<dyn Fn(&Message) + Send + Sync + 'static>>> {
    &self.listener.group_chat_created_handler
  }

  pub fn listen_supergroup_chat_created(&self) -> &Option<Arc<Box<dyn Fn(&Message) + Send + Sync + 'static>>> {
    &self.listener.supergroup_chat_created_handler
  }

  pub fn listen_channel_chat_create(&self) -> &Option<Arc<Box<dyn Fn(&Message) + Send + Sync + 'static>>> {
    &self.listener.channel_chat_created_handler
  }

  pub fn listen_migrate_to_chat(&self) -> &Option<Arc<Box<dyn Fn(&VMigrateToChatIdMessage) + Send + Sync + 'static>>> {
    &self.listener.migrate_to_chat_id_handler
  }

  pub fn listen_migrate_from_chat(&self) -> &Option<Arc<Box<dyn Fn(&VMigrateFromChatIdMessage) + Send + Sync + 'static>>> {
    &self.listener.migrate_from_chat_id_handler
  }

  pub fn listen_pinned(&self) -> &Option<Arc<Box<dyn Fn(&VPinnedMessageMessage) + Send + Sync + 'static>>> {
    &self.listener.pinned_message_handler
  }
}
