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
pub struct InlineResponse20012 {
  #[serde(rename = "status")]
  status: String,  // success 
  #[serde(rename = "from")]
  from: String,  // AUD 
  #[serde(rename = "to")]
  to: String,  // USD 
  #[serde(rename = "initialAmount")]
  initial_amount: f32,  // 100.0 
  #[serde(rename = "converted")]
  converted: f32,  // 78.76 
  #[serde(rename = "lastTrade")]
  last_trade: LastForexTrade, 
  #[serde(rename = "symbol")]
  symbol: Option<String>  // AUD/USD 
}

impl InlineResponse20012 {
  pub fn new(status: String, from: String, to: String, initial_amount: f32, converted: f32, last_trade: LastForexTrade, ) -> InlineResponse20012 {
    InlineResponse20012 {
      status: status,
      from: from,
      to: to,
      initial_amount: initial_amount,
      converted: converted,
      last_trade: last_trade,
      symbol: None
    }
  }

  pub fn set_status(&mut self, status: String) {
    self.status = status;
  }

  pub fn with_status(mut self, status: String) -> InlineResponse20012 {
    self.status = status;
    self
  }

  pub fn status(&self) -> &String {
    &self.status
  }


  pub fn set_from(&mut self, from: String) {
    self.from = from;
  }

  pub fn with_from(mut self, from: String) -> InlineResponse20012 {
    self.from = from;
    self
  }

  pub fn from(&self) -> &String {
    &self.from
  }


  pub fn set_to(&mut self, to: String) {
    self.to = to;
  }

  pub fn with_to(mut self, to: String) -> InlineResponse20012 {
    self.to = to;
    self
  }

  pub fn to(&self) -> &String {
    &self.to
  }


  pub fn set_initial_amount(&mut self, initial_amount: f32) {
    self.initial_amount = initial_amount;
  }

  pub fn with_initial_amount(mut self, initial_amount: f32) -> InlineResponse20012 {
    self.initial_amount = initial_amount;
    self
  }

  pub fn initial_amount(&self) -> &f32 {
    &self.initial_amount
  }


  pub fn set_converted(&mut self, converted: f32) {
    self.converted = converted;
  }

  pub fn with_converted(mut self, converted: f32) -> InlineResponse20012 {
    self.converted = converted;
    self
  }

  pub fn converted(&self) -> &f32 {
    &self.converted
  }


  pub fn set_last_trade(&mut self, last_trade: LastForexTrade) {
    self.last_trade = last_trade;
  }

  pub fn with_last_trade(mut self, last_trade: LastForexTrade) -> InlineResponse20012 {
    self.last_trade = last_trade;
    self
  }

  pub fn last_trade(&self) -> &LastForexTrade {
    &self.last_trade
  }


  pub fn set_symbol(&mut self, symbol: String) {
    self.symbol = Some(symbol);
  }

  pub fn with_symbol(mut self, symbol: String) -> InlineResponse20012 {
    self.symbol = Some(symbol);
    self
  }

  pub fn symbol(&self) -> Option<&String> {
    self.symbol.as_ref()
  }

  pub fn reset_symbol(&mut self) {
    self.symbol = None;
  }

}


