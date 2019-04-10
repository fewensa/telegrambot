
use std::mem;
use std::io::{self, Cursor};
use futures::{Future, Stream, Async};
use reqwest::r#async::{Client, Decoder};
use crate::{TGBotError, TGBotErrorKind};
use error_chain_mini::ErrorKind;

pub struct UpdatesStream {}


fn fetch() -> impl Future<Item=(), Error=()> {
  Client::new()
    .get("https://hyper.rs")
    .send()
    .and_then(|mut res| {
      println!("{}", res.status());

      let body = mem::replace(res.body_mut(), Decoder::empty());
      body.concat2()
    })
    .map_err(|err| println!("request error: {}", err))
    .map(|body| {
      let mut body = Cursor::new(body);
      let _ = io::copy(&mut body, &mut io::stdout())
        .map_err(|err| {
          println!("stdout error: {}", err);
        });
    })
}


impl Stream for UpdatesStream {
  type Item = String;
  type Error = TGBotError;

  fn poll(&mut self) -> Result<Async<Option<Self::Item>>, Self::Error> {
    let mut a = fetch();
    match futures::Future::poll(&mut a) {
//    match fetch().poll() {
      Ok(Async::Ready(value)) => {},
      Ok(Async::NotReady) => return Ok(Async::NotReady),
      Err(err) => return Err(TGBotErrorKind::Other.into_with(|| "Some err"))
    }

    Ok(Async::Ready(Some("abcd".to_string())))
  }
}


