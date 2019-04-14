use std::sync::Arc;

use error_chain_mini::ErrorKind;

use crate::{botrun, Config, TGBotErrorKind, TGBotResult};


pub enum Track {
  All,
  Message,
  Channel
}

#[derive(Debug)]
pub struct TelegramBot {
  cfg: Arc<Config>
}

impl TelegramBot {
  pub fn new(cfg: Config) -> TGBotResult<Self> {
    if cfg.token().is_empty() {
      return Err(TGBotErrorKind::LoseToken.into_with(|| "Telegram bot token is empty."));
    }
    Ok(TelegramBot {
      cfg: Arc::new(cfg)
    })
  }

  pub fn on_update(&self) -> &Self {
    self
  }

  pub fn on_callback_query(&self) -> &Self {
    self
  }

  pub fn on_command(&self, track: Track) -> &Self {
    self
  }

  pub fn on_text(&self, track: Track) -> &Self {
    self
  }

  pub fn on_error(&self) -> &Self {
    self
  }

  pub fn start(&self) -> TGBotResult<()> {
    botrun::run(self.cfg.clone())
  }
}
