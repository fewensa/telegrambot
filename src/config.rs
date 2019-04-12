use std::ffi::OsStr;

use error_chain_mini::ErrorKind;
use reqwest::Proxy;
use reqwest::r#async::{Client, ClientBuilder};

use crate::{TGBotErrorKind, TGBotResult};

#[derive(Debug, Clone)]
pub enum ConnectMode {
  Polling,
  Webhook,
}


#[derive(Debug)]
pub struct Config {
  token: String,
  mode: ConnectMode,
  proxy: Option<String>,
  client: Option<Client>,
}

impl Config {
  pub fn builder(token: String) -> ConfigBuilder {
    ConfigBuilder {
      token: Some(token),
      mode: ConnectMode::Polling,
      proxy: None,
    }
  }

  pub fn token(&self) -> String {
    self.token.clone()
  }

  pub fn mode(&self) -> ConnectMode {
    self.mode.clone()
  }

  pub fn client(&self) -> TGBotResult<Client> {
    Ok(self.client.clone().unwrap())
  }
}


pub struct ConfigBuilder {
  token: Option<String>,
  mode: ConnectMode,
  proxy: Option<String>,
}

impl ConfigBuilder {
  pub fn mode(mut self, mode: ConnectMode) -> Self {
    self.mode = mode;
    self
  }

  pub fn token(mut self, token: String) -> Self {
    self.token = Some(token);
    self
  }

  pub fn proxy<S>(mut self, proxy: S) -> Self where S: AsRef<OsStr> {
    self.proxy = Some(proxy.as_ref().to_str().unwrap().to_string());
    self
  }

  pub fn build(self) -> TGBotResult<Config> {
    let client = match self.proxy.clone() {
      Some(pxy) => {
        Client::builder()
          .proxy(Proxy::http(&pxy[..]).map_err(|err| TGBotErrorKind::ProxyError(err).into_err())?)
          .build().map_err(|err| TGBotErrorKind::ClientError(err).into_err())?
      }
      None => {
        Client::builder()
          .build().map_err(|err| TGBotErrorKind::ClientError(err).into_err())?
      }
    };

    Ok(Config {
      token: self.token.unwrap(),
      mode: self.mode,
      proxy: self.proxy,
      client: Some(client),
    })
  }
}

