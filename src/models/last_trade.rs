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
pub struct LastTrade {
  #[serde(rename = "price")]
  price: i64, 
  #[serde(rename = "size")]
  size: i64,  // 20 
  #[serde(rename = "exchange")]
  exchange: i64,  // 11 
  #[serde(rename = "cond1")]
  cond1: i64,  // 14 
  #[serde(rename = "cond2")]
  cond2: i64,  // 16 
  #[serde(rename = "cond3")]
  cond3: i64,  // 0 
  #[serde(rename = "cond4")]
  cond4: i64,  // 0 
  #[serde(rename = "timestamp")]
  timestamp: i64  // 1518086464720 
}

impl LastTrade {
  pub fn new(price: i64, size: i64, exchange: i64, cond1: i64, cond2: i64, cond3: i64, cond4: i64, timestamp: i64, ) -> LastTrade {
    LastTrade {
      price: price,
      size: size,
      exchange: exchange,
      cond1: cond1,
      cond2: cond2,
      cond3: cond3,
      cond4: cond4,
      timestamp: timestamp
    }
  }

  pub fn set_price(&mut self, price: i64) {
    self.price = price;
  }

  pub fn with_price(mut self, price: i64) -> LastTrade {
    self.price = price;
    self
  }

  pub fn price(&self) -> &i64 {
    &self.price
  }


  pub fn set_size(&mut self, size: i64) {
    self.size = size;
  }

  pub fn with_size(mut self, size: i64) -> LastTrade {
    self.size = size;
    self
  }

  pub fn size(&self) -> &i64 {
    &self.size
  }


  pub fn set_exchange(&mut self, exchange: i64) {
    self.exchange = exchange;
  }

  pub fn with_exchange(mut self, exchange: i64) -> LastTrade {
    self.exchange = exchange;
    self
  }

  pub fn exchange(&self) -> &i64 {
    &self.exchange
  }


  pub fn set_cond1(&mut self, cond1: i64) {
    self.cond1 = cond1;
  }

  pub fn with_cond1(mut self, cond1: i64) -> LastTrade {
    self.cond1 = cond1;
    self
  }

  pub fn cond1(&self) -> &i64 {
    &self.cond1
  }


  pub fn set_cond2(&mut self, cond2: i64) {
    self.cond2 = cond2;
  }

  pub fn with_cond2(mut self, cond2: i64) -> LastTrade {
    self.cond2 = cond2;
    self
  }

  pub fn cond2(&self) -> &i64 {
    &self.cond2
  }


  pub fn set_cond3(&mut self, cond3: i64) {
    self.cond3 = cond3;
  }

  pub fn with_cond3(mut self, cond3: i64) -> LastTrade {
    self.cond3 = cond3;
    self
  }

  pub fn cond3(&self) -> &i64 {
    &self.cond3
  }


  pub fn set_cond4(&mut self, cond4: i64) {
    self.cond4 = cond4;
  }

  pub fn with_cond4(mut self, cond4: i64) -> LastTrade {
    self.cond4 = cond4;
    self
  }

  pub fn cond4(&self) -> &i64 {
    &self.cond4
  }


  pub fn set_timestamp(&mut self, timestamp: i64) {
    self.timestamp = timestamp;
  }

  pub fn with_timestamp(mut self, timestamp: i64) -> LastTrade {
    self.timestamp = timestamp;
    self
  }

  pub fn timestamp(&self) -> &i64 {
    &self.timestamp
  }


}


