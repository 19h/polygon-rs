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
pub struct CryptoTick {
  #[serde(rename = "price")]
  price: i64, 
  #[serde(rename = "size")]
  size: i64, 
  #[serde(rename = "exchange")]
  exchange: i64,  // 2 
  #[serde(rename = "conditions")]
  conditions: Vec<i64>,  // [2,0] 
  #[serde(rename = "timestamp")]
  timestamp: i64  // 1525930460102 
}

impl CryptoTick {
  pub fn new(price: i64, size: i64, exchange: i64, conditions: Vec<i64>, timestamp: i64, ) -> CryptoTick {
    CryptoTick {
      price: price,
      size: size,
      exchange: exchange,
      conditions: conditions,
      timestamp: timestamp
    }
  }

  pub fn set_price(&mut self, price: i64) {
    self.price = price;
  }

  pub fn with_price(mut self, price: i64) -> CryptoTick {
    self.price = price;
    self
  }

  pub fn price(&self) -> &i64 {
    &self.price
  }


  pub fn set_size(&mut self, size: i64) {
    self.size = size;
  }

  pub fn with_size(mut self, size: i64) -> CryptoTick {
    self.size = size;
    self
  }

  pub fn size(&self) -> &i64 {
    &self.size
  }


  pub fn set_exchange(&mut self, exchange: i64) {
    self.exchange = exchange;
  }

  pub fn with_exchange(mut self, exchange: i64) -> CryptoTick {
    self.exchange = exchange;
    self
  }

  pub fn exchange(&self) -> &i64 {
    &self.exchange
  }


  pub fn set_conditions(&mut self, conditions: Vec<i64>) {
    self.conditions = conditions;
  }

  pub fn with_conditions(mut self, conditions: Vec<i64>) -> CryptoTick {
    self.conditions = conditions;
    self
  }

  pub fn conditions(&self) -> &Vec<i64> {
    &self.conditions
  }


  pub fn set_timestamp(&mut self, timestamp: i64) {
    self.timestamp = timestamp;
  }

  pub fn with_timestamp(mut self, timestamp: i64) -> CryptoTick {
    self.timestamp = timestamp;
    self
  }

  pub fn timestamp(&self) -> &i64 {
    &self.timestamp
  }


}


