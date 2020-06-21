/* 
 * Polygon API
 *
 * The future of fintech.
 *
 * OpenAPI spec version: 1.0.1

 */
#![allow(unused_imports)]
/// Dividend : Company dividend

use serde_json::Value;
use bigdecimal::BigDecimal;
use chrono::{NaiveDateTime, DateTime, FixedOffset, Utc};

use crate::models::*;
//use crate::date_serializer;
//use crate::datetime_serializer;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Dividend {
  #[serde(rename = "symbol")]
  symbol: StockSymbol, 
  #[serde(rename = "type")]
  _type: String,  // Dividend income 
  #[serde(rename = "exDate")]
  ex_date: DateTime<Utc>,  // 2016-11-03T04:00Z 
  #[serde(rename = "paymentDate")]
  payment_date: Option<DateTime<Utc>>,  // 2016-11-03T04:00Z 
  #[serde(rename = "recordDate")]
  record_date: Option<DateTime<Utc>>,  // 2016-11-03T04:00Z 
  #[serde(rename = "declaredDate")]
  declared_date: Option<DateTime<Utc>>,  // 2016-11-03T04:00Z 
  #[serde(rename = "amount")]
  amount: f32,  // 0.57 
  #[serde(rename = "qualified")]
  qualified: Option<String>,  // Q 
  #[serde(rename = "flag")]
  flag: Option<String>  // YE 
}

impl Dividend {
  pub fn new(symbol: StockSymbol, _type: String, ex_date: DateTime<Utc>, amount: f32, ) -> Dividend {
    Dividend {
      symbol: symbol,
      _type: _type,
      ex_date: ex_date,
      payment_date: None,
      record_date: None,
      declared_date: None,
      amount: amount,
      qualified: None,
      flag: None
    }
  }

  pub fn set_symbol(&mut self, symbol: StockSymbol) {
    self.symbol = symbol;
  }

  pub fn with_symbol(mut self, symbol: StockSymbol) -> Dividend {
    self.symbol = symbol;
    self
  }

  pub fn symbol(&self) -> &StockSymbol {
    &self.symbol
  }


  pub fn set__type(&mut self, _type: String) {
    self._type = _type;
  }

  pub fn with__type(mut self, _type: String) -> Dividend {
    self._type = _type;
    self
  }

  pub fn _type(&self) -> &String {
    &self._type
  }


  pub fn set_ex_date(&mut self, ex_date: DateTime<Utc>) {
    self.ex_date = ex_date;
  }

  pub fn with_ex_date(mut self, ex_date: DateTime<Utc>) -> Dividend {
    self.ex_date = ex_date;
    self
  }

  pub fn ex_date(&self) -> &DateTime<Utc> {
    &self.ex_date
  }


  pub fn set_payment_date(&mut self, payment_date: DateTime<Utc>) {
    self.payment_date = Some(payment_date);
  }

  pub fn with_payment_date(mut self, payment_date: DateTime<Utc>) -> Dividend {
    self.payment_date = Some(payment_date);
    self
  }

  pub fn payment_date(&self) -> Option<&DateTime<Utc>> {
    self.payment_date.as_ref()
  }

  pub fn reset_payment_date(&mut self) {
    self.payment_date = None;
  }

  pub fn set_record_date(&mut self, record_date: DateTime<Utc>) {
    self.record_date = Some(record_date);
  }

  pub fn with_record_date(mut self, record_date: DateTime<Utc>) -> Dividend {
    self.record_date = Some(record_date);
    self
  }

  pub fn record_date(&self) -> Option<&DateTime<Utc>> {
    self.record_date.as_ref()
  }

  pub fn reset_record_date(&mut self) {
    self.record_date = None;
  }

  pub fn set_declared_date(&mut self, declared_date: DateTime<Utc>) {
    self.declared_date = Some(declared_date);
  }

  pub fn with_declared_date(mut self, declared_date: DateTime<Utc>) -> Dividend {
    self.declared_date = Some(declared_date);
    self
  }

  pub fn declared_date(&self) -> Option<&DateTime<Utc>> {
    self.declared_date.as_ref()
  }

  pub fn reset_declared_date(&mut self) {
    self.declared_date = None;
  }

  pub fn set_amount(&mut self, amount: f32) {
    self.amount = amount;
  }

  pub fn with_amount(mut self, amount: f32) -> Dividend {
    self.amount = amount;
    self
  }

  pub fn amount(&self) -> &f32 {
    &self.amount
  }


  pub fn set_qualified(&mut self, qualified: String) {
    self.qualified = Some(qualified);
  }

  pub fn with_qualified(mut self, qualified: String) -> Dividend {
    self.qualified = Some(qualified);
    self
  }

  pub fn qualified(&self) -> Option<&String> {
    self.qualified.as_ref()
  }

  pub fn reset_qualified(&mut self) {
    self.qualified = None;
  }

  pub fn set_flag(&mut self, flag: String) {
    self.flag = Some(flag);
  }

  pub fn with_flag(mut self, flag: String) -> Dividend {
    self.flag = Some(flag);
    self
  }

  pub fn flag(&self) -> Option<&String> {
    self.flag.as_ref()
  }

  pub fn reset_flag(&mut self) {
    self.flag = None;
  }

}


