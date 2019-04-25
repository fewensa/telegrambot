use serde::{Deserialize, Deserializer};

use crate::advanced;
use crate::types::{Chat, Contact, RawMessage, User};
use crate::vision::message::*;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct PossibilityMessage {
  raw: RawMessage
}


impl<'de> Deserialize<'de> for PossibilityMessage {
  fn deserialize<D>(deserializer: D) -> Result<PossibilityMessage, D::Error>
    where D: Deserializer<'de>
  {
    let raw: RawMessage = Deserialize::deserialize(deserializer)?;
    Ok(PossibilityMessage::new(raw))
  }
}


impl PossibilityMessage {
  pub fn new(raw: RawMessage) -> Self {
    PossibilityMessage {
      raw
    }
  }

  fn to_message(&self) -> Message {
    advanced::to_message(&self.raw, false)
  }

  pub fn is_group(&self) -> bool {
    if let Chat::Group(_) = &self.raw.chat { true } else { false }
  }

  pub fn is_supergroup(&self) -> bool {
    if let Chat::Supergroup(_) = &self.raw.chat { true } else { false }
  }

  pub fn is_private(&self) -> bool {
    if let Chat::Private(_) = &self.raw.chat { true } else { false }
  }

  pub fn is_channel(&self) -> bool {
    if let Chat::Channel(_) = &self.raw.chat { true } else { false }
  }

  pub fn is_forward(&self) -> bool {
    self.raw.forward_from != &None
  }

  pub fn is_new_chat_title(&self) -> bool {
    self.raw.new_chat_title != &None
  }

  pub fn is_new_chat_photo(&self) -> bool {
    self.raw.new_chat_photo != &None
  }

  pub fn is_delete_chat_photo(&self) -> bool {
    self.raw.delete_chat_photo != &None
  }

  pub fn is_group_chat_created(&self) -> bool {
    self.raw.group_chat_created != &None
  }

  pub fn is_supergroup_chat_created(&self) -> bool {
    self.raw.supergroup_chat_created != &None
  }

  pub fn is_channel_chat_created(&self) -> bool {
    self.raw.channel_chat_created != &None
  }

  pub fn is_pinned_message(&self) -> bool {
    self.raw.pinned_message != &None
  }

  pub fn is_forward_user_hidden(&self) -> bool {
    self.raw.forward_sender_name != &None
  }

  pub fn is_reply(&self) -> bool {
    self.raw.reply_to_message != &None
  }

  pub fn is_text(&self) -> bool {
    self.raw.text != &None
  }

  pub fn is_audio(&self) -> bool {
    self.raw.audio != &None
  }

  pub fn is_document(&self) -> bool {
    self.raw.document != &None
  }

  pub fn is_photo(&self) -> bool {
    self.raw.photo != &None
  }

  pub fn is_sticker(&self) -> bool {
    self.raw.sticker != &None
  }

  pub fn is_video(&self) -> bool {
    self.raw.video != &None
  }

  pub fn is_voice(&self) -> bool {
    self.raw.voice != &None
  }

  pub fn is_video_note(&self) -> bool {
    self.raw.video_note != &None
  }

  pub fn is_location(&self) -> bool {
    self.raw.location != &None
  }

  pub fn is_venue(&self) -> bool {
    self.raw.venue != &None
  }

  pub fn is_contact(&self) -> bool {
    self.raw.contact != &None
  }

  pub fn from(&self) -> Option<User> {
    self.raw.from.clone()
  }


  pub fn with_raw<F>(&self, fnc: F) -> &Self where F: Fn(&RawMessage) {
    fnc(&self.raw);
    self
  }

  pub fn with_message<F>(&self, fnc: F) -> &Self where F: Fn(&Message) {
    let message = advanced::to_message(&self.raw, false);
    fnc(&message);
    self
  }

  pub fn with_contact<F>(&self, fnc: F) -> &Self where F: Fn(&Contact) {
    if let Some(contact) = &self.raw.contact {
      let message = self.to_message();
      let entities = self.raw.entities.clone().unwrap_or_else(|| Vec::with_capacity(0));
      let obj = VContactMessage { message, contact: contact.clone() };
      fnc(&obj);
    }
    self
  }

  pub fn with_text<F>(&self, fnc: F) -> &Self where F: Fn(&VTextMessage) {
    if let Some(text) = &self.raw.text {
      let message = self.to_message();
      let entities = self.raw.entities.clone().unwrap_or_else(|| Vec::with_capacity(0));
      let obj = VTextMessage { message, text: text.clone(), entities };
      fnc(&obj);
    }
    self
  }

  pub fn with_audio<F>(&self, fnc: F) -> &Self where F: Fn(&VAudioMessage) {
    if let Some(audio) = &self.raw.audio {
      let message = self.to_message();
      let obj = VAudioMessage { message, audio: audio.clone() };
      fnc(&obj);
    }
    self
  }

  pub fn with_document<F>(&self, fnc: F) -> &Self where F: Fn(&VDocumentMessage) {
    if let Some(document) = &self.raw.document {
      let message = self.to_message();
      let obj = VDocumentMessage {
        message,
        document: document.clone(),
        caption: self.raw.caption.clone(),
      };
      fnc(&obj);
    }
    self
  }

  pub fn with_photo<F>(&self, fnc: F) -> &Self where F: Fn(&VPhotoMessage) {
    if let Some(photo) = &self.raw.photo {
      let message = self.to_message();
      let obj = VPhotoMessage {
        message,
        photo: photo.clone(),
        caption: self.raw.caption.clone(),
        media_group_id: self.raw.media_group_id.clone(),
      };
      fnc(&obj);
    }
    self
  }

  pub fn with_sticker<F>(&self, fnc: F) -> &Self where F: Fn(&VStickerMessage) {
    if let Some(sticker) = &self.raw.sticker {
      let message = self.to_message();
      let obj = VStickerMessage { message, sticker: sticker.clone() };
      fnc(&obj);
    }
    self
  }

  pub fn with_video<F>(&self, fnc: F) -> &Self where F: Fn(&VVideoMessage) {
    if let Some(video) = &self.raw.video {
      let message = self.to_message();
      let obj = VVideoMessage {
        message,
        video: video.clone(),
        caption: self.raw.caption.clone(),
        media_group_id: self.raw.media_group_id.clone(),
      };
      fnc(&obj);
    }
    self
  }

  pub fn with_voice<F>(&self, fnc: F) -> &Self where F: Fn(&VVoiceMessage) {
    if let Some(voice) = &self.raw.voice {
      let message = self.to_message();
      let obj = VVoiceMessage { message, voice: voice.clone() };
      fnc(&obj);
    }
    self
  }

  pub fn with_video_note<F>(&self, fnc: F) -> &Self where F: Fn(&VVideoNoteMessage) {
    if let Some(video_note) = &self.raw.video_note {
      let message = self.to_message();
      let obj = VVideoNoteMessage { message, video_note: video_note.clone() };
      fnc(&obj);
    }
    self
  }


  pub fn with_location<F>(&self, fnc: F) -> &Self where F: Fn(&VLocationMessage) {
    if let Some(location) = &self.raw.location {
      let message = self.to_message();
      let obj = VLocationMessage { message, location: location.clone() };
      fnc(&obj);
    }
    self
  }

  pub fn with_venue<F>(&self, fnc: F) -> &Self where F: Fn(&VVenueMessage) {
    if let Some(venue) = &self.raw.venue {
      let message = self.to_message();
      let obj = VVenueMessage { message, venue: venue.clone() };
      fnc(&obj);
    }
    self
  }
}
