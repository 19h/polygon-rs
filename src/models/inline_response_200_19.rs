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
pub struct InlineResponse20019 {
  #[serde(rename = "status")]
  status: String,  // OK 
  #[serde(rename = "ticker")]
  ticker: Option<CryptoSnapshotTicker> 
}

impl InlineResponse20019 {
  pub fn new(status: String, ) -> InlineResponse20019 {
    InlineResponse20019 {
      status: status,
      ticker: None
    }
  }

  pub fn set_status(&mut self, status: String) {
    self.status = status;
  }

  pub fn with_status(mut self, status: String) -> InlineResponse20019 {
    self.status = status;
    self
  }

  pub fn status(&self) -> &String {
    &self.status
  }


  pub fn set_ticker(&mut self, ticker: CryptoSnapshotTicker) {
    self.ticker = Some(ticker);
  }

  pub fn with_ticker(mut self, ticker: CryptoSnapshotTicker) -> InlineResponse20019 {
    self.ticker = Some(ticker);
    self
  }

  pub fn ticker(&self) -> Option<&CryptoSnapshotTicker> {
    self.ticker.as_ref()
  }

  pub fn reset_ticker(&mut self) {
    self.ticker = None;
  }

}


