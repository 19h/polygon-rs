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
pub struct MarketStatusCurrencies {
  #[serde(rename = "fx")]
  fx: Option<String>,  // open 
  #[serde(rename = "crypto")]
  crypto: Option<String>  // open 
}

impl MarketStatusCurrencies {
  pub fn new() -> MarketStatusCurrencies {
    MarketStatusCurrencies {
      fx: None,
      crypto: None
    }
  }

  pub fn set_fx(&mut self, fx: String) {
    self.fx = Some(fx);
  }

  pub fn with_fx(mut self, fx: String) -> MarketStatusCurrencies {
    self.fx = Some(fx);
    self
  }

  pub fn fx(&self) -> Option<&String> {
    self.fx.as_ref()
  }

  pub fn reset_fx(&mut self) {
    self.fx = None;
  }

  pub fn set_crypto(&mut self, crypto: String) {
    self.crypto = Some(crypto);
  }

  pub fn with_crypto(mut self, crypto: String) -> MarketStatusCurrencies {
    self.crypto = Some(crypto);
    self
  }

  pub fn crypto(&self) -> Option<&String> {
    self.crypto.as_ref()
  }

  pub fn reset_crypto(&mut self) {
    self.crypto = None;
  }

}


