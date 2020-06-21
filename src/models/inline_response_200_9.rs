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
pub struct InlineResponse2009 {
  #[serde(rename = "status")]
  status: String,  // OK 
  #[serde(rename = "tickers")]
  tickers: Vec<StocksSnapshotTicker> 
}

impl InlineResponse2009 {
  pub fn new(status: String, tickers: Vec<StocksSnapshotTicker>, ) -> InlineResponse2009 {
    InlineResponse2009 {
      status: status,
      tickers: tickers
    }
  }

  pub fn set_status(&mut self, status: String) {
    self.status = status;
  }

  pub fn with_status(mut self, status: String) -> InlineResponse2009 {
    self.status = status;
    self
  }

  pub fn status(&self) -> &String {
    &self.status
  }


  pub fn set_tickers(&mut self, tickers: Vec<StocksSnapshotTicker>) {
    self.tickers = tickers;
  }

  pub fn with_tickers(mut self, tickers: Vec<StocksSnapshotTicker>) -> InlineResponse2009 {
    self.tickers = tickers;
    self
  }

  pub fn tickers(&self) -> &Vec<StocksSnapshotTicker> {
    &self.tickers
  }


}


