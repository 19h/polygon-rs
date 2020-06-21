/* 
 * Polygon API
 *
 * The future of fintech.
 *
 * OpenAPI spec version: 1.0.1

 */
#![allow(unused_imports)]
/// Unauthorized : Unauthorized - Check our API Key and account status

use serde_json::Value;
use bigdecimal::BigDecimal;
use chrono::{NaiveDateTime, DateTime, FixedOffset, Utc};

use crate::models::*;
//use crate::date_serializer;
//use crate::datetime_serializer;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Unauthorized {
  #[serde(rename = "message")]
  message: Option<String> 
}

impl Unauthorized {
  pub fn new() -> Unauthorized {
    Unauthorized {
      message: None
    }
  }

  pub fn set_message(&mut self, message: String) {
    self.message = Some(message);
  }

  pub fn with_message(mut self, message: String) -> Unauthorized {
    self.message = Some(message);
    self
  }

  pub fn message(&self) -> Option<&String> {
    self.message.as_ref()
  }

  pub fn reset_message(&mut self) {
    self.message = None;
  }

}


