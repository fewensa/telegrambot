use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;

use crate::advanced::Track;
use crate::types::Update;
use crate::vision::TextMessage;

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum ListenerType {
  Text,
  Command,
  CallbackQuery,
  Update,
}

#[derive(Clone)]
pub struct Listener {
  text_handler: HashMap<Track, Arc<Box<dyn Fn((&TextMessage, bool)) + Send + Sync + 'static>>>,
  update_handler: Option<Arc<Box<dyn Fn(&Update) + Send + Sync + 'static>>>,
}

impl Default for Listener {
  fn default() -> Self {
    Listener {
      text_handler: HashMap::new(),
      update_handler: None,
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

  pub fn on_text_message<F>(&mut self, fnc: F) -> &mut Self where F: Fn((&TextMessage, bool)) + Send + Sync + 'static {
    self.text_handler.insert(Track::Message, Arc::new(Box::new(fnc)));
    self
  }

//  pub fn on_channel_post_text_message

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

  pub fn listen_text(&self, track: Track) -> Option<&Arc<Box<dyn Fn((&TextMessage, bool)) + Send + Sync + 'static>>> {
    self.listener.text_handler.get(&track)
  }

  pub fn listen_update(&self) -> &Option<Arc<Box<dyn Fn(&Update) + Send + Sync + 'static>>> {
    &self.listener.update_handler
  }
}
