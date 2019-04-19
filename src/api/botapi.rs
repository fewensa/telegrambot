use futures::future::Future;

use crate::api::answer_callback_query::AnswerCallbackQuery;
use crate::api::delete_message::DeleteMessage;
use crate::api::edit_message_caption::EditMessageCaption;
use crate::api::edit_message_live_location::EditMessageLiveLocation;
use crate::api::edit_message_reply_markup::EditMessageReplyMarkup;
use crate::api::edit_message_text::EditMessageText;
use crate::api::forward_message::ForwardMessage;
use crate::api::get_chat::GetChat;
use crate::api::get_chat_administrators::GetChatAdministrators;
use crate::api::get_chat_member::GetChatMember;
use crate::api::get_chat_members_count::GetChatMembersCount;
use crate::api::get_file::GetFile;
use crate::api::get_me::GetMe;
use crate::api::get_updates::GetUpdates;
use crate::api::get_user_profile_photos::GetUserProfilePhotos;
use crate::api::kick_chat_member::KickChatMember;
use crate::api::leave_chat::LeaveChat;
use crate::api::pin_chat_message::PinChatMessage;
use crate::api::rawreq::RawReq;
use crate::api::req::TGReq;
use crate::api::resp::JsonTrueToUnitResp;
use crate::api::send_audio::SendAudio;
use crate::api::send_chat_action::SendChatAction;
use crate::api::send_contact::SendContact;
use crate::api::send_location::SendLocation;
use crate::api::send_message::SendMessage;
use crate::api::send_venue::SendVenue;
use crate::api::stop_message_live_location::StopMessageLiveLocation;
use crate::api::TGResp;
use crate::api::unban_chat_member::UnbanChatMember;
use crate::api::unpin_chat_message::UnpinChatMessage;
use crate::errors::{TGBotError, TGBotResult};
use crate::tgfut::TGFuture;
use crate::tglog;
use crate::types::*;
use crate::vision::PossibilityMessage;

//pub const TELEGRAM_API_URL: &'static str = "https://tgb.akafwtll.tk/";
pub const TELEGRAM_API_URL: &'static str = "https://api.telegram.org/";

pub fn spawn<F>(fut: F) where F: Future<Item = (), Error = ()> + 'static + Send {
  tokio::spawn(fut);
}

#[derive(Debug, Clone)]
pub struct BotApi {
  rawreq: RawReq,
}

impl BotApi {
  pub fn new(rawreq: RawReq) -> Self {
    BotApi {
      rawreq
    }
  }

  fn send<Req: TGReq>(&self, req: &Req) -> TGFuture<Option<<Req::Resp as TGResp>::Type>> {
    let request = futures::future::result(req.request());
    let rawreq = self.rawreq.clone();
    let response = request.and_then(move |httpreq| {
      rawreq.request(httpreq)
    });
    let future = response
      .map(move |resp| {
        let dez: Result<<Req::Resp as TGResp>::Type, TGBotError> = Req::Resp::deserialize(resp);
        match dez {
          Ok(ret) => Some(ret),
          Err(err) => {
            // todo: if error do more thing
            error!(tglog::telegram(), "Call telegram api fail: {:?}", err);
            None
          }
        }
      }).map_err(|e| e);
    TGFuture::new(Box::new(future))
  }


  pub fn get_update(&self, get_updates: &GetUpdates) -> TGFuture<Option<Vec<Update>>> {
    self.send(get_updates)
  }

  pub fn get_me(&self) -> TGFuture<Option<User>> {
    self.send(&GetMe)
  }

  pub fn answer_callback_query(&self, req: &AnswerCallbackQuery) -> TGFuture<Option<()>> {
    self.send(req)
  }

  pub fn delete_message(&self, req: &DeleteMessage) -> TGFuture<Option<()>> {
    self.send(req)
  }

  pub fn edit_message_caption(&self, req: &EditMessageCaption) -> TGFuture<Option<PossibilityMessage>> {
    self.send(req)
  }

  pub fn edit_message_reply_markup(&self, req: &EditMessageReplyMarkup) -> TGFuture<Option<PossibilityMessage>> {
    self.send(req)
  }

  pub fn edit_message_text(&self, req: &EditMessageText) -> TGFuture<Option<PossibilityMessage>> {
    self.send(req)
  }

  pub fn forward_message(&self, req: &ForwardMessage) -> TGFuture<Option<PossibilityMessage>> {
    self.send(req)
  }

  pub fn get_chat(&self, req: &GetChat) -> TGFuture<Option<Chat>> {
    self.send(req)
  }

  pub fn get_chat_administrators(&self, req: &GetChatAdministrators) -> TGFuture<Option<Vec<ChatMember>>> {
    self.send(req)
  }

  pub fn get_chat_member(&self, req: &GetChatMember) -> TGFuture<Option<ChatMember>> {
    self.send(req)
  }

  pub fn get_chat_members_count(&self, req: &GetChatMembersCount) -> TGFuture<Option<i64>> {
    self.send(req)
  }

  pub fn get_file(&self, req: &GetFile) -> TGFuture<Option<File>> {
    self.send(req)
  }

  pub fn get_user_profile_photos(&self, req: &GetUserProfilePhotos) -> TGFuture<Option<UserProfilePhotos>> {
    self.send(req)
  }

  pub fn kick_chat_member(&self, req: &KickChatMember) -> TGFuture<Option<()>> {
    self.send(req)
  }

  pub fn leave_chat(&self, req: &LeaveChat) -> TGFuture<Option<()>> {
    self.send(req)
  }

  pub fn pin_chat_message(&self, req: &PinChatMessage) -> TGFuture<Option<()>> {
    self.send(req)
  }

  pub fn send_audio(&self, req: &SendAudio) -> TGFuture<Option<()>> {
    self.send(req)
  }

  pub fn send_chat_action(&self, req: &SendChatAction) -> TGFuture<Option<()>> {
    self.send(req)
  }

  pub fn send_contact(&self, req: &SendContact) -> TGFuture<Option<PossibilityMessage>> {
    self.send(req)
  }

  pub fn send_location(&self, req: &SendLocation) -> TGFuture<Option<PossibilityMessage>> {
    self.send(req)
  }

  pub fn send_message(&self, req: &SendMessage) -> TGFuture<Option<PossibilityMessage>> {
    self.send(req)
  }

  pub fn send_venue(&self, req: &SendVenue) -> TGFuture<Option<PossibilityMessage>> {
    self.send(req)
  }

  pub fn stop_message_live_location(&self, req: &StopMessageLiveLocation) -> TGFuture<Option<PossibilityMessage>> {
    self.send(req)
  }

  pub fn unban_chat_member(&self, req: &UnbanChatMember) -> TGFuture<Option<()>> {
    self.send(req)
  }

  pub fn unpin_chat_message(&self, req: &UnpinChatMessage) -> TGFuture<Option<()>> {
    self.send(req)
  }

}

