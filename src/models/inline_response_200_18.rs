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
pub struct InlineResponse20018 {
  #[serde(rename = "symbol")]
  symbol: String,  // BTC-USD 
  #[serde(rename = "isUTC")]
  is_utc: Option<bool>,  // true 
  #[serde(rename = "day")]
  day: Option<String>,  // Wed May 09 02:00:00 CEST 2018 
  #[serde(rename = "open")]
  open: Option<f32>,  // 9228.1801 
  #[serde(rename = "close")]
  close: Option<f32>,  // 9300.40000001 
  #[serde(rename = "openTrades")]
  open_trades: Option<Vec<CryptoTickJson>>, 
  #[serde(rename = "closingTrades")]
  closing_trades: Option<Vec<CryptoTickJson>> 
}

impl InlineResponse20018 {
  pub fn new(symbol: String, ) -> InlineResponse20018 {
    InlineResponse20018 {
      symbol: symbol,
      is_utc: None,
      day: None,
      open: None,
      close: None,
      open_trades: None,
      closing_trades: None
    }
  }

  pub fn set_symbol(&mut self, symbol: String) {
    self.symbol = symbol;
  }

  pub fn with_symbol(mut self, symbol: String) -> InlineResponse20018 {
    self.symbol = symbol;
    self
  }

  pub fn symbol(&self) -> &String {
    &self.symbol
  }


  pub fn set_is_utc(&mut self, is_utc: bool) {
    self.is_utc = Some(is_utc);
  }

  pub fn with_is_utc(mut self, is_utc: bool) -> InlineResponse20018 {
    self.is_utc = Some(is_utc);
    self
  }

  pub fn is_utc(&self) -> Option<&bool> {
    self.is_utc.as_ref()
  }

  pub fn reset_is_utc(&mut self) {
    self.is_utc = None;
  }

  pub fn set_day(&mut self, day: String) {
    self.day = Some(day);
  }

  pub fn with_day(mut self, day: String) -> InlineResponse20018 {
    self.day = Some(day);
    self
  }

  pub fn day(&self) -> Option<&String> {
    self.day.as_ref()
  }

  pub fn reset_day(&mut self) {
    self.day = None;
  }

  pub fn set_open(&mut self, open: f32) {
    self.open = Some(open);
  }

  pub fn with_open(mut self, open: f32) -> InlineResponse20018 {
    self.open = Some(open);
    self
  }

  pub fn open(&self) -> Option<&f32> {
    self.open.as_ref()
  }

  pub fn reset_open(&mut self) {
    self.open = None;
  }

  pub fn set_close(&mut self, close: f32) {
    self.close = Some(close);
  }

  pub fn with_close(mut self, close: f32) -> InlineResponse20018 {
    self.close = Some(close);
    self
  }

  pub fn close(&self) -> Option<&f32> {
    self.close.as_ref()
  }

  pub fn reset_close(&mut self) {
    self.close = None;
  }

  pub fn set_open_trades(&mut self, open_trades: Vec<CryptoTickJson>) {
    self.open_trades = Some(open_trades);
  }

  pub fn with_open_trades(mut self, open_trades: Vec<CryptoTickJson>) -> InlineResponse20018 {
    self.open_trades = Some(open_trades);
    self
  }

  pub fn open_trades(&self) -> Option<&Vec<CryptoTickJson>> {
    self.open_trades.as_ref()
  }

  pub fn reset_open_trades(&mut self) {
    self.open_trades = None;
  }

  pub fn set_closing_trades(&mut self, closing_trades: Vec<CryptoTickJson>) {
    self.closing_trades = Some(closing_trades);
  }

  pub fn with_closing_trades(mut self, closing_trades: Vec<CryptoTickJson>) -> InlineResponse20018 {
    self.closing_trades = Some(closing_trades);
    self
  }

  pub fn closing_trades(&self) -> Option<&Vec<CryptoTickJson>> {
    self.closing_trades.as_ref()
  }

  pub fn reset_closing_trades(&mut self) {
    self.closing_trades = None;
  }

}


