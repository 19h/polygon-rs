/* 
 * Polygon API
 *
 * The future of fintech.
 *
 * OpenAPI spec version: 1.0.1
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */
#![allow(unused_imports)]

use serde_json::Value;
use bigdecimal::BigDecimal;
use chrono::{NaiveDateTime, DateTime, FixedOffset, Utc};

use crate::models::*;
//use crate::date_serializer;
//use crate::datetime_serializer;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LastForexTrade {
  #[serde(rename = "price")]
  price: f32,  // 0.78131 
  #[serde(rename = "exchange")]
  exchange: i64,  // 20 
  #[serde(rename = "timestamp")]
  timestamp: i64  // 1518086429487 
}

impl LastForexTrade {
  pub fn new(price: f32, exchange: i64, timestamp: i64, ) -> LastForexTrade {
    LastForexTrade {
      price: price,
      exchange: exchange,
      timestamp: timestamp
    }
  }

  pub fn set_price(&mut self, price: f32) {
    self.price = price;
  }

  pub fn with_price(mut self, price: f32) -> LastForexTrade {
    self.price = price;
    self
  }

  pub fn price(&self) -> &f32 {
    &self.price
  }


  pub fn set_exchange(&mut self, exchange: i64) {
    self.exchange = exchange;
  }

  pub fn with_exchange(mut self, exchange: i64) -> LastForexTrade {
    self.exchange = exchange;
    self
  }

  pub fn exchange(&self) -> &i64 {
    &self.exchange
  }


  pub fn set_timestamp(&mut self, timestamp: i64) {
    self.timestamp = timestamp;
  }

  pub fn with_timestamp(mut self, timestamp: i64) -> LastForexTrade {
    self.timestamp = timestamp;
    self
  }

  pub fn timestamp(&self) -> &i64 {
    &self.timestamp
  }


}


