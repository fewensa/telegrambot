use std::sync::Arc;

use error_chain_mini::ErrorKind;

use crate::botrun;
use crate::config::Config;
use crate::errors::{TGBotErrorKind, TGBotResult};
use crate::listener::{Listener, Lout};
use crate::types::Update;
use crate::vision::*;
use crate::api::BotApi;

pub struct TelegramBot {
  cfg: Arc<Config>,
  listener: Listener,
}

impl TelegramBot {
  pub fn new(cfg: Config) -> TGBotResult<Self> {
    if cfg.token().is_empty() {
      return Err(TGBotErrorKind::LoseToken.into_with(|| "Telegram bot token is empty."));
    }
    let cfg = Arc::new(cfg);
    Ok(TelegramBot {
      cfg,
      listener: Listener::default(),
    })
  }

  pub fn on_start<F>(&mut self, fnc: F) -> &mut Self where
    F: Fn(&VCommand) + Send + Sync + 'static {
    self.listener.on_command("/start", fnc);
    self
  }

  pub fn on_command<S, F>(&mut self, command: S, fnc: F) -> &mut Self where
    S: AsRef<str> + 'static,
    F: Fn(&VCommand) + Send + Sync + 'static {
    self.listener.on_command(command, fnc);
    self
  }

  pub fn on_callback_query<F>(&mut self, fnc: F) -> &mut Self where F: Fn(&VCallbackQuery) + Send + Sync + 'static {
    self.listener.on_callback_query(fnc);
    self
  }

  pub fn on_error<F>(&mut self, fnc: F) -> &mut Self where F: Fn(&String) + Send + Sync + 'static {
    self.listener.on_error(fnc);
    self
  }


  pub fn on_update<F>(&mut self, fnc: F) -> &mut Self where F: Fn(&Update) + Send + Sync + 'static {
    self.listener.on_update(fnc);
    self
  }

  pub fn on_text<F>(&mut self, fnc: F) -> &mut Self where F: Fn(&VTextMessage) + Send + Sync + 'static {
    self.listener.on_text(fnc);
    self
  }

  pub fn on_audio<F>(&mut self, fnc: F) -> &mut Self where F: Fn(&VAudioMessage) + Send + Sync + 'static {
    self.listener.on_audio(fnc);
    self
  }


  pub fn on_document<F>(&mut self, fnc: F) -> &mut Self where F: Fn(&VDocumentMessage) + Send + Sync + 'static {
    self.listener.on_document(fnc);
    self
  }

  pub fn on_photo<F>(&mut self, fnc: F) -> &mut Self where F: Fn(&VPhotoMessage) + Send + Sync + 'static {
    self.listener.on_photo(fnc);
    self
  }

  pub fn on_sticker<F>(&mut self, fnc: F) -> &mut Self where F: Fn(&VStickerMessage) + Send + Sync + 'static {
    self.listener.on_sticker(fnc);
    self
  }

  pub fn on_video<F>(&mut self, fnc: F) -> &mut Self where F: Fn(&VVideoMessage) + Send + Sync + 'static {
    self.listener.on_video(fnc);
    self
  }

  pub fn on_voice<F>(&mut self, fnc: F) -> &mut Self where F: Fn(&VVoiceMessage) + Send + Sync + 'static {
    self.listener.on_voice(fnc);
    self
  }

  pub fn on_video_note<F>(&mut self, fnc: F) -> &mut Self where F: Fn(&VVideoNoteMessage) + Send + Sync + 'static {
    self.listener.on_video_note(fnc);
    self
  }

  pub fn on_contact<F>(&mut self, fnc: F) -> &mut Self where F: Fn(&VContactMessage) + Send + Sync + 'static {
    self.listener.on_contact(fnc);
    self
  }

  pub fn on_location<F>(&mut self, fnc: F) -> &mut Self where F: Fn(&VLocationMessage) + Send + Sync + 'static {
    self.listener.on_location(fnc);
    self
  }

  pub fn on_venue<F>(&mut self, fnc: F) -> &mut Self where F: Fn(&VVenueMessage) + Send + Sync + 'static {
    self.listener.on_venue(fnc);
    self
  }

  pub fn on_new_chat_members<F>(&mut self, fnc: F) -> &mut Self where F: Fn(&VNewChatMembersMessage) + Send + Sync + 'static {
    self.listener.on_new_chat_members(fnc);
    self
  }

  pub fn on_left_chat_member<F>(&mut self, fnc: F) -> &mut Self where F: Fn(&VLeftChatMemberMessage) + Send + Sync + 'static {
    self.listener.on_left_chat_member(fnc);
    self
  }

  pub fn on_new_chat_title<F>(&mut self, fnc: F) -> &mut Self where F: Fn(&VChatTitleMessage) + Send + Sync + 'static {
    self.listener.on_new_chat_title(fnc);
    self
  }

  pub fn on_new_chat_photo<F>(&mut self, fnc: F) -> &mut Self where F: Fn(&VChatPhotoMessage) + Send + Sync + 'static {
    self.listener.on_new_chat_photo(fnc);
    self
  }

  pub fn on_delete_chat_photo<F>(&mut self, fnc: F) -> &mut Self where F: Fn(&Message) + Send + Sync + 'static {
    self.listener.on_delete_chat_photo(fnc);
    self
  }

  pub fn on_group_chat_created<F>(&mut self, fnc: F) -> &mut Self where F: Fn(&Message) + Send + Sync + 'static {
    self.listener.on_group_chat_created(fnc);
    self
  }

  pub fn on_supergroup_chat_created<F>(&mut self, fnc: F) -> &mut Self where F: Fn(&Message) + Send + Sync + 'static {
    self.listener.on_supergroup_chat_created(fnc);
    self
  }

  pub fn on_channel_chat_create<F>(&mut self, fnc: F) -> &mut Self where F: Fn(&Message) + Send + Sync + 'static {
    self.listener.on_channel_chat_create(fnc);
    self
  }

  pub fn on_migrate_to_chat<F>(&mut self, fnc: F) -> &mut Self where F: Fn(&VMigrateToChatIdMessage) + Send + Sync + 'static {
    self.listener.on_migrate_to_chat(fnc);
    self
  }

  pub fn on_migrate_from_chat<F>(&mut self, fnc: F) -> &mut Self where F: Fn(&VMigrateFromChatIdMessage) + Send + Sync + 'static {
    self.listener.on_migrate_from_chat(fnc);
    self
  }

  pub fn on_pinned<F>(&mut self, fnc: F) -> &mut Self where F: Fn(&VPinnedMessageMessage) + Send + Sync + 'static {
    self.listener.on_pinned(fnc);
    self
  }


  pub fn start(&self) -> TGBotResult<()> {
    let lout = Lout::new(self.listener.clone());
    if let Err(error) = botrun::run(self.cfg.clone(), Arc::new(lout)) {
      return Err(error);
    }
    Ok(())
  }
}


