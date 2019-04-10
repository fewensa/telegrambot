/// This object represents an incoming update.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Update {
  /// The update‘s unique identifier. Update identifiers start from a certain
  /// positive number and increase sequentially.
  pub id: i64,
  /// Kind of the incoming update.
  pub kind: UpdateKind,
}

/// Kind of the incoming update.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum UpdateKind {
  //  /// New incoming message of any kind — text, photo, sticker, etc.
//  Message(Message),
//  /// New version of a message that is known to the bot and was edited
//  EditedMessage(Message),
//  /// New incoming channel post of any kind — text, photo, sticker, etc.
//  ChannelPost(ChannelPost),
//  /// New version of a channel post that is known to the bot and was edited
//  EditedChannelPost(ChannelPost),
//  // InlineQuery(InlineQuery),
//  // ChosenInlineResult(ChosenInlineResult),
//  CallbackQuery(CallbackQuery),
  #[doc(hidden)]
  Error(String),
  #[doc(hidden)]
  Unknown,
}
