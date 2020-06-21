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
pub struct InlineResponse20013 {
  #[serde(rename = "status")]
  status: String,  // success 
  #[serde(rename = "symbol")]
  symbol: String,  // AUD/USD 
  #[serde(rename = "last")]
  last: LastForexQuote 
}

impl InlineResponse20013 {
  pub fn new(status: String, symbol: String, last: LastForexQuote, ) -> InlineResponse20013 {
    InlineResponse20013 {
      status: status,
      symbol: symbol,
      last: last
    }
  }

  pub fn set_status(&mut self, status: String) {
    self.status = status;
  }

  pub fn with_status(mut self, status: String) -> InlineResponse20013 {
    self.status = status;
    self
  }

  pub fn status(&self) -> &String {
    &self.status
  }


  pub fn set_symbol(&mut self, symbol: String) {
    self.symbol = symbol;
  }

  pub fn with_symbol(mut self, symbol: String) -> InlineResponse20013 {
    self.symbol = symbol;
    self
  }

  pub fn symbol(&self) -> &String {
    &self.symbol
  }


  pub fn set_last(&mut self, last: LastForexQuote) {
    self.last = last;
  }

  pub fn with_last(mut self, last: LastForexQuote) -> InlineResponse20013 {
    self.last = last;
    self
  }

  pub fn last(&self) -> &LastForexQuote {
    &self.last
  }


}


