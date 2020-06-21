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
pub struct InlineResponse20016 {
  #[serde(rename = "symbol")]
  symbol: String,  // BTC-USD 
  #[serde(rename = "isUTC")]
  is_utc: Option<bool>,  // true 
  #[serde(rename = "day")]
  day: Option<String>,  // Wed May 09 00:00:00 GMT 2018 
  #[serde(rename = "openTrades")]
  open_trades: Option<Vec<CryptoTickJson>>, 
  #[serde(rename = "closingTrades")]
  closing_trades: Option<Vec<CryptoTickJson>> 
}

impl InlineResponse20016 {
  pub fn new(symbol: String, ) -> InlineResponse20016 {
    InlineResponse20016 {
      symbol: symbol,
      is_utc: None,
      day: None,
      open_trades: None,
      closing_trades: None
    }
  }

  pub fn set_symbol(&mut self, symbol: String) {
    self.symbol = symbol;
  }

  pub fn with_symbol(mut self, symbol: String) -> InlineResponse20016 {
    self.symbol = symbol;
    self
  }

  pub fn symbol(&self) -> &String {
    &self.symbol
  }


  pub fn set_is_utc(&mut self, is_utc: bool) {
    self.is_utc = Some(is_utc);
  }

  pub fn with_is_utc(mut self, is_utc: bool) -> InlineResponse20016 {
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

  pub fn with_day(mut self, day: String) -> InlineResponse20016 {
    self.day = Some(day);
    self
  }

  pub fn day(&self) -> Option<&String> {
    self.day.as_ref()
  }

  pub fn reset_day(&mut self) {
    self.day = None;
  }

  pub fn set_open_trades(&mut self, open_trades: Vec<CryptoTickJson>) {
    self.open_trades = Some(open_trades);
  }

  pub fn with_open_trades(mut self, open_trades: Vec<CryptoTickJson>) -> InlineResponse20016 {
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

  pub fn with_closing_trades(mut self, closing_trades: Vec<CryptoTickJson>) -> InlineResponse20016 {
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


