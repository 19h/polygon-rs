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
pub struct InlineResponse2008 {
  #[serde(rename = "status")]
  status: String,  // success 
  #[serde(rename = "symbol")]
  symbol: String,  // AAPL 
  #[serde(rename = "last")]
  last: LastQuote 
}

impl InlineResponse2008 {
  pub fn new(status: String, symbol: String, last: LastQuote, ) -> InlineResponse2008 {
    InlineResponse2008 {
      status: status,
      symbol: symbol,
      last: last
    }
  }

  pub fn set_status(&mut self, status: String) {
    self.status = status;
  }

  pub fn with_status(mut self, status: String) -> InlineResponse2008 {
    self.status = status;
    self
  }

  pub fn status(&self) -> &String {
    &self.status
  }


  pub fn set_symbol(&mut self, symbol: String) {
    self.symbol = symbol;
  }

  pub fn with_symbol(mut self, symbol: String) -> InlineResponse2008 {
    self.symbol = symbol;
    self
  }

  pub fn symbol(&self) -> &String {
    &self.symbol
  }


  pub fn set_last(&mut self, last: LastQuote) {
    self.last = last;
  }

  pub fn with_last(mut self, last: LastQuote) -> InlineResponse2008 {
    self.last = last;
    self
  }

  pub fn last(&self) -> &LastQuote {
    &self.last
  }


}


