use std::sync::Arc;

use error_chain_mini::ErrorKind;

use crate::botrun;
use crate::config::Config;
use crate::errors::{TGBotErrorKind, TGBotResult};
use crate::listener::{Listener, Lout};
use crate::types::Update;
use crate::vision::TextMessage;

pub struct TelegramBot {
  cfg: Arc<Config>,
  listener: Listener,
}

impl TelegramBot {
  pub fn new(cfg: Config) -> TGBotResult<Self> {
    if cfg.token().is_empty() {
      return Err(TGBotErrorKind::LoseToken.into_with(|| "Telegram bot token is empty."));
    }
    Ok(TelegramBot {
      cfg: Arc::new(cfg),
      listener: Listener::default(),
    })
  }

  pub fn on_update<F>(&mut self, fnc: F) -> &mut Self where F: Fn(&Update) + Send + Sync + 'static {
    self.listener.on_update(fnc);
    self
  }

  pub fn on_text_message<F>(&mut self, fnc: F) -> &mut Self where F: Fn((&TextMessage, bool)) + Send + Sync + 'static {
    self.listener.on_text_message(fnc);
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


