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
pub struct StocksOpenClose {
  #[serde(rename = "status")]
  status: String,  // OK 
  #[serde(rename = "from")]
  from: String,  // Wed Jun 03 00:00:00 GMT 2020 
  #[serde(rename = "symbol")]
  symbol: String,  // NVDA 
  #[serde(rename = "open")]
  open: i64, 
  #[serde(rename = "high")]
  high: i64, 
  #[serde(rename = "low")]
  low: i64, 
  #[serde(rename = "close")]
  close: i64, 
  #[serde(rename = "volume")]
  volume: i64,  // 9194990 
  #[serde(rename = "preMarket")]
  pre_market: i64,  // 355 
  #[serde(rename = "afterHours")]
  after_hours: i64 
}

impl StocksOpenClose {
  pub fn new(status: String, from: String, symbol: String, open: i64, high: i64, low: i64, close: i64, volume: i64, pre_market: i64, after_hours: i64, ) -> StocksOpenClose {
    StocksOpenClose {
      status: status,
      from: from,
      symbol: symbol,
      open: open,
      high: high,
      low: low,
      close: close,
      volume: volume,
      pre_market: pre_market,
      after_hours: after_hours
    }
  }

  pub fn set_status(&mut self, status: String) {
    self.status = status;
  }

  pub fn with_status(mut self, status: String) -> StocksOpenClose {
    self.status = status;
    self
  }

  pub fn status(&self) -> &String {
    &self.status
  }


  pub fn set_from(&mut self, from: String) {
    self.from = from;
  }

  pub fn with_from(mut self, from: String) -> StocksOpenClose {
    self.from = from;
    self
  }

  pub fn from(&self) -> &String {
    &self.from
  }


  pub fn set_symbol(&mut self, symbol: String) {
    self.symbol = symbol;
  }

  pub fn with_symbol(mut self, symbol: String) -> StocksOpenClose {
    self.symbol = symbol;
    self
  }

  pub fn symbol(&self) -> &String {
    &self.symbol
  }


  pub fn set_open(&mut self, open: i64) {
    self.open = open;
  }

  pub fn with_open(mut self, open: i64) -> StocksOpenClose {
    self.open = open;
    self
  }

  pub fn open(&self) -> &i64 {
    &self.open
  }


  pub fn set_high(&mut self, high: i64) {
    self.high = high;
  }

  pub fn with_high(mut self, high: i64) -> StocksOpenClose {
    self.high = high;
    self
  }

  pub fn high(&self) -> &i64 {
    &self.high
  }


  pub fn set_low(&mut self, low: i64) {
    self.low = low;
  }

  pub fn with_low(mut self, low: i64) -> StocksOpenClose {
    self.low = low;
    self
  }

  pub fn low(&self) -> &i64 {
    &self.low
  }


  pub fn set_close(&mut self, close: i64) {
    self.close = close;
  }

  pub fn with_close(mut self, close: i64) -> StocksOpenClose {
    self.close = close;
    self
  }

  pub fn close(&self) -> &i64 {
    &self.close
  }


  pub fn set_volume(&mut self, volume: i64) {
    self.volume = volume;
  }

  pub fn with_volume(mut self, volume: i64) -> StocksOpenClose {
    self.volume = volume;
    self
  }

  pub fn volume(&self) -> &i64 {
    &self.volume
  }


  pub fn set_pre_market(&mut self, pre_market: i64) {
    self.pre_market = pre_market;
  }

  pub fn with_pre_market(mut self, pre_market: i64) -> StocksOpenClose {
    self.pre_market = pre_market;
    self
  }

  pub fn pre_market(&self) -> &i64 {
    &self.pre_market
  }


  pub fn set_after_hours(&mut self, after_hours: i64) {
    self.after_hours = after_hours;
  }

  pub fn with_after_hours(mut self, after_hours: i64) -> StocksOpenClose {
    self.after_hours = after_hours;
    self
  }

  pub fn after_hours(&self) -> &i64 {
    &self.after_hours
  }


}


