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
pub struct LastForexQuote {
  #[serde(rename = "ask")]
  ask: i64, 
  #[serde(rename = "bid")]
  bid: i64, 
  #[serde(rename = "exchange")]
  exchange: i64,  // 20 
  #[serde(rename = "timestamp")]
  timestamp: i64  // 1518086520785 
}

impl LastForexQuote {
  pub fn new(ask: i64, bid: i64, exchange: i64, timestamp: i64, ) -> LastForexQuote {
    LastForexQuote {
      ask: ask,
      bid: bid,
      exchange: exchange,
      timestamp: timestamp
    }
  }

  pub fn set_ask(&mut self, ask: i64) {
    self.ask = ask;
  }

  pub fn with_ask(mut self, ask: i64) -> LastForexQuote {
    self.ask = ask;
    self
  }

  pub fn ask(&self) -> &i64 {
    &self.ask
  }


  pub fn set_bid(&mut self, bid: i64) {
    self.bid = bid;
  }

  pub fn with_bid(mut self, bid: i64) -> LastForexQuote {
    self.bid = bid;
    self
  }

  pub fn bid(&self) -> &i64 {
    &self.bid
  }


  pub fn set_exchange(&mut self, exchange: i64) {
    self.exchange = exchange;
  }

  pub fn with_exchange(mut self, exchange: i64) -> LastForexQuote {
    self.exchange = exchange;
    self
  }

  pub fn exchange(&self) -> &i64 {
    &self.exchange
  }


  pub fn set_timestamp(&mut self, timestamp: i64) {
    self.timestamp = timestamp;
  }

  pub fn with_timestamp(mut self, timestamp: i64) -> LastForexQuote {
    self.timestamp = timestamp;
    self
  }

  pub fn timestamp(&self) -> &i64 {
    &self.timestamp
  }


}


