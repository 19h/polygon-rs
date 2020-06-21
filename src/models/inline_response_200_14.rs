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
pub struct InlineResponse20014 {
  #[serde(rename = "status")]
  status: String,  // OK 
  #[serde(rename = "tickers")]
  tickers: Vec<ForexSnapshotTicker> 
}

impl InlineResponse20014 {
  pub fn new(status: String, tickers: Vec<ForexSnapshotTicker>, ) -> InlineResponse20014 {
    InlineResponse20014 {
      status: status,
      tickers: tickers
    }
  }

  pub fn set_status(&mut self, status: String) {
    self.status = status;
  }

  pub fn with_status(mut self, status: String) -> InlineResponse20014 {
    self.status = status;
    self
  }

  pub fn status(&self) -> &String {
    &self.status
  }


  pub fn set_tickers(&mut self, tickers: Vec<ForexSnapshotTicker>) {
    self.tickers = tickers;
  }

  pub fn with_tickers(mut self, tickers: Vec<ForexSnapshotTicker>) -> InlineResponse20014 {
    self.tickers = tickers;
    self
  }

  pub fn tickers(&self) -> &Vec<ForexSnapshotTicker> {
    &self.tickers
  }


}


