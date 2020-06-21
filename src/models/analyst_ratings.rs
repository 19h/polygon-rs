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
pub struct AnalystRatings {
  #[serde(rename = "symbol")]
  symbol: String,  // AAPL 
  #[serde(rename = "analysts")]
  analysts: f32,  // 27.0 
  #[serde(rename = "change")]
  change: f32,  // -0.04 
  #[serde(rename = "strongBuy")]
  strong_buy: Value, 
  #[serde(rename = "buy")]
  buy: Value, 
  #[serde(rename = "hold")]
  hold: Value, 
  #[serde(rename = "sell")]
  sell: Value, 
  #[serde(rename = "strongSell")]
  strong_sell: Value, 
  #[serde(rename = "updated")]
  updated: DateTime<Utc> 
}

impl AnalystRatings {
  pub fn new(symbol: String, analysts: f32, change: f32, strong_buy: Value, buy: Value, hold: Value, sell: Value, strong_sell: Value, updated: DateTime<Utc>, ) -> AnalystRatings {
    AnalystRatings {
      symbol: symbol,
      analysts: analysts,
      change: change,
      strong_buy: strong_buy,
      buy: buy,
      hold: hold,
      sell: sell,
      strong_sell: strong_sell,
      updated: updated
    }
  }

  pub fn set_symbol(&mut self, symbol: String) {
    self.symbol = symbol;
  }

  pub fn with_symbol(mut self, symbol: String) -> AnalystRatings {
    self.symbol = symbol;
    self
  }

  pub fn symbol(&self) -> &String {
    &self.symbol
  }


  pub fn set_analysts(&mut self, analysts: f32) {
    self.analysts = analysts;
  }

  pub fn with_analysts(mut self, analysts: f32) -> AnalystRatings {
    self.analysts = analysts;
    self
  }

  pub fn analysts(&self) -> &f32 {
    &self.analysts
  }


  pub fn set_change(&mut self, change: f32) {
    self.change = change;
  }

  pub fn with_change(mut self, change: f32) -> AnalystRatings {
    self.change = change;
    self
  }

  pub fn change(&self) -> &f32 {
    &self.change
  }


  pub fn set_strong_buy(&mut self, strong_buy: Value) {
    self.strong_buy = strong_buy;
  }

  pub fn with_strong_buy(mut self, strong_buy: Value) -> AnalystRatings {
    self.strong_buy = strong_buy;
    self
  }

  pub fn strong_buy(&self) -> &Value {
    &self.strong_buy
  }


  pub fn set_buy(&mut self, buy: Value) {
    self.buy = buy;
  }

  pub fn with_buy(mut self, buy: Value) -> AnalystRatings {
    self.buy = buy;
    self
  }

  pub fn buy(&self) -> &Value {
    &self.buy
  }


  pub fn set_hold(&mut self, hold: Value) {
    self.hold = hold;
  }

  pub fn with_hold(mut self, hold: Value) -> AnalystRatings {
    self.hold = hold;
    self
  }

  pub fn hold(&self) -> &Value {
    &self.hold
  }


  pub fn set_sell(&mut self, sell: Value) {
    self.sell = sell;
  }

  pub fn with_sell(mut self, sell: Value) -> AnalystRatings {
    self.sell = sell;
    self
  }

  pub fn sell(&self) -> &Value {
    &self.sell
  }


  pub fn set_strong_sell(&mut self, strong_sell: Value) {
    self.strong_sell = strong_sell;
  }

  pub fn with_strong_sell(mut self, strong_sell: Value) -> AnalystRatings {
    self.strong_sell = strong_sell;
    self
  }

  pub fn strong_sell(&self) -> &Value {
    &self.strong_sell
  }


  pub fn set_updated(&mut self, updated: DateTime<Utc>) {
    self.updated = updated;
  }

  pub fn with_updated(mut self, updated: DateTime<Utc>) -> AnalystRatings {
    self.updated = updated;
    self
  }

  pub fn updated(&self) -> &DateTime<Utc> {
    &self.updated
  }


}


