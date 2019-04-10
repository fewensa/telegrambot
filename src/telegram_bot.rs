use std::ffi::OsStr;

use error_chain_mini::ErrorKind;

use crate::{botrun, TGBotErrorKind, TGBotResult};

#[derive(Debug)]
pub enum ConnectMode {
  Polling,
  Webhook,
}

#[derive(Debug)]
pub struct Config {
  pub token: String,
  pub mode: ConnectMode,
}

#[derive(Debug)]
pub struct TelegramBot {
  cfg: Config
}

impl TelegramBot {
  pub fn new(cfg: Config) -> TGBotResult<Self> {
    if cfg.token.is_empty() {
      return Err(TGBotErrorKind::LoseToken.into_with(|| "Telegram bot token is empty."));
    }
    Ok(TelegramBot {
      cfg
    })
  }

  pub fn command(&self) -> &Self {
    self
  }

  pub fn start(&self) -> TGBotResult<()> {
    botrun::run(&self.cfg)
  }
}
