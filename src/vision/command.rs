use std::num::ParseIntError;
use std::str::ParseBoolError;

use crate::types::MessageEntity;
use crate::vision::message::Message;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct VCommand {
  pub message: Message,
  pub text: String,
  pub entities: Vec<MessageEntity>,
  pub command: String,
  pub args: Vec<String>,
}


// todo: Need it?
//impl VCommand {
//  pub fn arg(&self, ix: u32) -> Option<String> {
//    self.args.get(ix)
//  }
//
//  pub fn arg_as_i32(&self, ix: u32) -> Result<Option<i32>, ParseIntError> {
//    Ok(self.arg(ix).map(|item| item.parse::<i32>()?))
//  }
//
//  pub fn arg_as_i64(&self, ix: u32) -> Result<Option<i64>, ParseIntError> {
//    Ok(self.arg(ix).map(|item| item.parse::<i64>()?))
//  }
//
//  pub fn arg_as_isize(&self, ix: u32) -> Result<Option<isize>, ParseIntError> {
//    Ok(self.arg(ix).map(|item| item.parse::<isize>()?))
//  }
//
//  pub fn arg_as_u32(&self, ix: u32) -> Result<Option<u32>, ParseIntError> {
//    Ok(self.arg(ix).map(|item| item.parse::<u32>()?))
//  }
//
//  pub fn arg_as_u64(&self, ix: u32) -> Result<Option<u64>, ParseIntError> {
//    Ok(self.arg(ix).map(|item| item.parse::<u64>()?))
//  }
//
//  pub fn arg_as_usize(&self, ix: u32) -> Result<Option<usize>, ParseIntError> {
//    Ok(self.arg(ix).map(|item| item.parse::<usize>()?))
//  }
//
//  pub fn arg_as_bool(&self, ix: u32) -> Result<Option<bool>, ParseBoolError> {
//    Ok(self.arg(ix).map(|item| item.parse::<bool>()?))
//  }
//}

