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
pub struct HistTrade {
  #[serde(rename = "condition1")]
  condition1: i64,  // 14 
  #[serde(rename = "condition2")]
  condition2: i64,  // 12 
  #[serde(rename = "condition3")]
  condition3: i64,  // 0 
  #[serde(rename = "condition4")]
  condition4: i64,  // 0 
  #[serde(rename = "exchange")]
  exchange: String,  // 12 
  #[serde(rename = "price")]
  price: i64, 
  #[serde(rename = "size")]
  size: i64,  // 50 
  #[serde(rename = "timestamp")]
  timestamp: String  // 2018-03-05T14:30:00.080Z 
}

impl HistTrade {
  pub fn new(condition1: i64, condition2: i64, condition3: i64, condition4: i64, exchange: String, price: i64, size: i64, timestamp: String, ) -> HistTrade {
    HistTrade {
      condition1: condition1,
      condition2: condition2,
      condition3: condition3,
      condition4: condition4,
      exchange: exchange,
      price: price,
      size: size,
      timestamp: timestamp
    }
  }

  pub fn set_condition1(&mut self, condition1: i64) {
    self.condition1 = condition1;
  }

  pub fn with_condition1(mut self, condition1: i64) -> HistTrade {
    self.condition1 = condition1;
    self
  }

  pub fn condition1(&self) -> &i64 {
    &self.condition1
  }


  pub fn set_condition2(&mut self, condition2: i64) {
    self.condition2 = condition2;
  }

  pub fn with_condition2(mut self, condition2: i64) -> HistTrade {
    self.condition2 = condition2;
    self
  }

  pub fn condition2(&self) -> &i64 {
    &self.condition2
  }


  pub fn set_condition3(&mut self, condition3: i64) {
    self.condition3 = condition3;
  }

  pub fn with_condition3(mut self, condition3: i64) -> HistTrade {
    self.condition3 = condition3;
    self
  }

  pub fn condition3(&self) -> &i64 {
    &self.condition3
  }


  pub fn set_condition4(&mut self, condition4: i64) {
    self.condition4 = condition4;
  }

  pub fn with_condition4(mut self, condition4: i64) -> HistTrade {
    self.condition4 = condition4;
    self
  }

  pub fn condition4(&self) -> &i64 {
    &self.condition4
  }


  pub fn set_exchange(&mut self, exchange: String) {
    self.exchange = exchange;
  }

  pub fn with_exchange(mut self, exchange: String) -> HistTrade {
    self.exchange = exchange;
    self
  }

  pub fn exchange(&self) -> &String {
    &self.exchange
  }


  pub fn set_price(&mut self, price: i64) {
    self.price = price;
  }

  pub fn with_price(mut self, price: i64) -> HistTrade {
    self.price = price;
    self
  }

  pub fn price(&self) -> &i64 {
    &self.price
  }


  pub fn set_size(&mut self, size: i64) {
    self.size = size;
  }

  pub fn with_size(mut self, size: i64) -> HistTrade {
    self.size = size;
    self
  }

  pub fn size(&self) -> &i64 {
    &self.size
  }


  pub fn set_timestamp(&mut self, timestamp: String) {
    self.timestamp = timestamp;
  }

  pub fn with_timestamp(mut self, timestamp: String) -> HistTrade {
    self.timestamp = timestamp;
    self
  }

  pub fn timestamp(&self) -> &String {
    &self.timestamp
  }


}


