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
pub struct InlineResponse20015 {
  #[serde(rename = "status")]
  status: String,  // success 
  #[serde(rename = "symbol")]
  symbol: String,  // BTC-USD 
  #[serde(rename = "last")]
  last: CryptoTick, 
  #[serde(rename = "lastAverage")]
  last_average: Option<Value> 
}

impl InlineResponse20015 {
  pub fn new(status: String, symbol: String, last: CryptoTick, ) -> InlineResponse20015 {
    InlineResponse20015 {
      status: status,
      symbol: symbol,
      last: last,
      last_average: None
    }
  }

  pub fn set_status(&mut self, status: String) {
    self.status = status;
  }

  pub fn with_status(mut self, status: String) -> InlineResponse20015 {
    self.status = status;
    self
  }

  pub fn status(&self) -> &String {
    &self.status
  }


  pub fn set_symbol(&mut self, symbol: String) {
    self.symbol = symbol;
  }

  pub fn with_symbol(mut self, symbol: String) -> InlineResponse20015 {
    self.symbol = symbol;
    self
  }

  pub fn symbol(&self) -> &String {
    &self.symbol
  }


  pub fn set_last(&mut self, last: CryptoTick) {
    self.last = last;
  }

  pub fn with_last(mut self, last: CryptoTick) -> InlineResponse20015 {
    self.last = last;
    self
  }

  pub fn last(&self) -> &CryptoTick {
    &self.last
  }


  pub fn set_last_average(&mut self, last_average: Value) {
    self.last_average = Some(last_average);
  }

  pub fn with_last_average(mut self, last_average: Value) -> InlineResponse20015 {
    self.last_average = Some(last_average);
    self
  }

  pub fn last_average(&self) -> Option<&Value> {
    self.last_average.as_ref()
  }

  pub fn reset_last_average(&mut self) {
    self.last_average = None;
  }

}


