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
pub struct Symbol {
  #[serde(rename = "symbol")]
  symbol: StockSymbol, 
  #[serde(rename = "name")]
  name: String,  // Apple Inc 
  #[serde(rename = "type")]
  _type: String,  // cs 
  #[serde(rename = "url")]
  url: String,  // https://api.polygon.io/v1/meta/symbols/AAPL 
  #[serde(rename = "updated")]
  updated: DateTime<Utc>, 
  #[serde(rename = "isOTC")]
  is_otc: bool  // false 
}

impl Symbol {
  pub fn new(symbol: StockSymbol, name: String, _type: String, url: String, updated: DateTime<Utc>, is_otc: bool, ) -> Symbol {
    Symbol {
      symbol: symbol,
      name: name,
      _type: _type,
      url: url,
      updated: updated,
      is_otc: is_otc
    }
  }

  pub fn set_symbol(&mut self, symbol: StockSymbol) {
    self.symbol = symbol;
  }

  pub fn with_symbol(mut self, symbol: StockSymbol) -> Symbol {
    self.symbol = symbol;
    self
  }

  pub fn symbol(&self) -> &StockSymbol {
    &self.symbol
  }


  pub fn set_name(&mut self, name: String) {
    self.name = name;
  }

  pub fn with_name(mut self, name: String) -> Symbol {
    self.name = name;
    self
  }

  pub fn name(&self) -> &String {
    &self.name
  }


  pub fn set__type(&mut self, _type: String) {
    self._type = _type;
  }

  pub fn with__type(mut self, _type: String) -> Symbol {
    self._type = _type;
    self
  }

  pub fn _type(&self) -> &String {
    &self._type
  }


  pub fn set_url(&mut self, url: String) {
    self.url = url;
  }

  pub fn with_url(mut self, url: String) -> Symbol {
    self.url = url;
    self
  }

  pub fn url(&self) -> &String {
    &self.url
  }


  pub fn set_updated(&mut self, updated: DateTime<Utc>) {
    self.updated = updated;
  }

  pub fn with_updated(mut self, updated: DateTime<Utc>) -> Symbol {
    self.updated = updated;
    self
  }

  pub fn updated(&self) -> &DateTime<Utc> {
    &self.updated
  }


  pub fn set_is_otc(&mut self, is_otc: bool) {
    self.is_otc = is_otc;
  }

  pub fn with_is_otc(mut self, is_otc: bool) -> Symbol {
    self.is_otc = is_otc;
    self
  }

  pub fn is_otc(&self) -> &bool {
    &self.is_otc
  }


}


