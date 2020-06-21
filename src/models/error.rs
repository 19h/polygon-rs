/* 
 * Polygon API
 *
 * The future of fintech.
 *
 * OpenAPI spec version: 1.0.1

 */
#![allow(unused_imports)]

use serde_json::Value;
use bigdecimal::BigDecimal;
use chrono::{NaiveDateTime, DateTime, FixedOffset, Utc};

use crate::models::*;
//use crate::date_serializer;
//use crate::datetime_serializer;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Error {
  #[serde(rename = "code")]
  code: Option<i64>, 
  #[serde(rename = "message")]
  message: Option<String>, 
  #[serde(rename = "fields")]
  fields: Option<String> 
}

impl Error {
  pub fn new() -> Error {
    Error {
      code: None,
      message: None,
      fields: None
    }
  }

  pub fn set_code(&mut self, code: i64) {
    self.code = Some(code);
  }

  pub fn with_code(mut self, code: i64) -> Error {
    self.code = Some(code);
    self
  }

  pub fn code(&self) -> Option<&i64> {
    self.code.as_ref()
  }

  pub fn reset_code(&mut self) {
    self.code = None;
  }

  pub fn set_message(&mut self, message: String) {
    self.message = Some(message);
  }

  pub fn with_message(mut self, message: String) -> Error {
    self.message = Some(message);
    self
  }

  pub fn message(&self) -> Option<&String> {
    self.message.as_ref()
  }

  pub fn reset_message(&mut self) {
    self.message = None;
  }

  pub fn set_fields(&mut self, fields: String) {
    self.fields = Some(fields);
  }

  pub fn with_fields(mut self, fields: String) -> Error {
    self.fields = Some(fields);
    self
  }

  pub fn fields(&self) -> Option<&String> {
    self.fields.as_ref()
  }

  pub fn reset_fields(&mut self) {
    self.fields = None;
  }

}


