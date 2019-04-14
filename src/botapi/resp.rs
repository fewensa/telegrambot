use error_chain_mini::ErrorKind;
use serde::de::{Deserialize, DeserializeOwned, Deserializer, Error};

use crate::{TGBotError, TGBotErrorKind, True};

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct HttpResp {
  pub body: Option<Vec<u8>>
}


pub trait TGResp {
  type Type;

  fn deserialize(resp: HttpResp) -> Result<Self::Type, TGBotError>;
}

pub struct RespType<Type> {
  phantom: ::std::marker::PhantomData<Type>,
}


pub trait JsonResp {
  type Raw;
  type Type;

  fn map(raw: Self::Raw) -> Self::Type;
}

impl<Type> JsonResp for RespType<Type> {
  type Raw = Type;
  type Type = Type;

  fn map(raw: Self::Raw) -> Self::Type {
    raw
  }
}


pub struct JsonTrueToUnitResp;

impl JsonResp for JsonTrueToUnitResp {
  type Raw = True;
  type Type = ();

  fn map(_: Self::Raw) -> Self::Type {
    ()
  }
}


impl<R: JsonResp> TGResp for R where <R as JsonResp>::Raw: DeserializeOwned {
  type Type = <R as JsonResp>::Type;

  fn deserialize(resp: HttpResp) -> Result<Self::Type, TGBotError> {
    if let Some(body) = resp.body.as_ref() {
      match serde_json::from_slice(body) {
        Ok(raw) => {
          match raw {
            RespWrapper::Success { result } => {
              Ok(<Self as JsonResp>::map(result))
            }
            RespWrapper::Error { description, parameters } => {
              Err(TGBotErrorKind::TelegramError(description, parameters).into_err())
            }
          }
        }
        Err(error) => Err(TGBotErrorKind::JsonError(error).into_err())
      }
    } else {
      Err(TGBotErrorKind::EmptyBody.into_err())
    }
  }
}


/// All API responses are from this type. Mostly used internal.
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum RespWrapper<T> {
  /// Request was successful.
  Success {
    /// Response result.
    result: T,
  },
  /// Request was unsuccessful.
  Error {
    /// Human-readable description of the result.
    description: String,
    /// Contains information about why a request was unsuccessful.
    parameters: Option<RespParas>,
  },
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for RespWrapper<T> {
  fn deserialize<D>(deserializer: D) -> Result<RespWrapper<T>, D::Error> where D: Deserializer<'de> {
    let raw: RawResp<T> = Deserialize::deserialize(deserializer)?;
    match (raw.ok, raw.description, raw.result) {
      (false, Some(description), None) => {
        Ok(RespWrapper::Error {
          description,
          parameters: raw.parameters,
        })
      }

      (true, None, Some(result)) => Ok(RespWrapper::Success { result }),

      _ => Err(D::Error::custom("ambiguous response")),
    }
  }
}

/// Directly mapped telegram API response.
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Deserialize)]
pub struct RawResp<T> {
  /// If ‘ok’ equals true, the request was successful.
  ok: bool,
  /// Human-readable description of the result.
  description: Option<String>,
  /// Result of the query.
  result: Option<T>,
  /// Information about why a request was unsuccessful.
  parameters: Option<RespParas>,
}

/// Contains information about why a request was unsuccessful.
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Deserialize)]
pub struct RespParas {
  /// The group has been migrated to a supergroup with the specified identifier.
  pub migrate_to_chat_id: Option<i64>,
  /// In case of exceeding flood control, the number of seconds left to wait
  /// before the request can be repeated.
  pub retry_after: Option<i64>,
}
