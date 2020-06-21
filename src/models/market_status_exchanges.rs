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
pub struct MarketStatusExchanges {
  #[serde(rename = "nyse")]
  nyse: Option<String>,  // open 
  #[serde(rename = "nasdaq")]
  nasdaq: Option<String>,  // open 
  #[serde(rename = "otc")]
  otc: Option<String>  // extended-hours 
}

impl MarketStatusExchanges {
  pub fn new() -> MarketStatusExchanges {
    MarketStatusExchanges {
      nyse: None,
      nasdaq: None,
      otc: None
    }
  }

  pub fn set_nyse(&mut self, nyse: String) {
    self.nyse = Some(nyse);
  }

  pub fn with_nyse(mut self, nyse: String) -> MarketStatusExchanges {
    self.nyse = Some(nyse);
    self
  }

  pub fn nyse(&self) -> Option<&String> {
    self.nyse.as_ref()
  }

  pub fn reset_nyse(&mut self) {
    self.nyse = None;
  }

  pub fn set_nasdaq(&mut self, nasdaq: String) {
    self.nasdaq = Some(nasdaq);
  }

  pub fn with_nasdaq(mut self, nasdaq: String) -> MarketStatusExchanges {
    self.nasdaq = Some(nasdaq);
    self
  }

  pub fn nasdaq(&self) -> Option<&String> {
    self.nasdaq.as_ref()
  }

  pub fn reset_nasdaq(&mut self) {
    self.nasdaq = None;
  }

  pub fn set_otc(&mut self, otc: String) {
    self.otc = Some(otc);
  }

  pub fn with_otc(mut self, otc: String) -> MarketStatusExchanges {
    self.otc = Some(otc);
    self
  }

  pub fn otc(&self) -> Option<&String> {
    self.otc.as_ref()
  }

  pub fn reset_otc(&mut self) {
    self.otc = None;
  }

}


