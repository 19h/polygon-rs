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
pub struct InlineResponse20010 {
  #[serde(rename = "status")]
  status: String,  // OK 
  #[serde(rename = "ticker")]
  ticker: Option<StocksSnapshotTicker> 
}

impl InlineResponse20010 {
  pub fn new(status: String, ) -> InlineResponse20010 {
    InlineResponse20010 {
      status: status,
      ticker: None
    }
  }

  pub fn set_status(&mut self, status: String) {
    self.status = status;
  }

  pub fn with_status(mut self, status: String) -> InlineResponse20010 {
    self.status = status;
    self
  }

  pub fn status(&self) -> &String {
    &self.status
  }


  pub fn set_ticker(&mut self, ticker: StocksSnapshotTicker) {
    self.ticker = Some(ticker);
  }

  pub fn with_ticker(mut self, ticker: StocksSnapshotTicker) -> InlineResponse20010 {
    self.ticker = Some(ticker);
    self
  }

  pub fn ticker(&self) -> Option<&StocksSnapshotTicker> {
    self.ticker.as_ref()
  }

  pub fn reset_ticker(&mut self) {
    self.ticker = None;
  }

}


