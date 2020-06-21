/* 
 * Polygon API
 *
 * The future of fintech.
 *
 * OpenAPI spec version: 1.0.1

 */
#![allow(unused_imports)]
/// Split : Symbol split

use serde_json::Value;
use bigdecimal::BigDecimal;
use chrono::{NaiveDateTime, DateTime, FixedOffset, Utc};

use crate::models::*;
//use crate::date_serializer;
//use crate::datetime_serializer;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Split {
  #[serde(rename = "ticker")]
  ticker: TickerSymbol, 
  #[serde(rename = "exDate")]
  ex_date: DateTime<Utc>, 
  #[serde(rename = "paymentDate")]
  payment_date: DateTime<Utc>, 
  #[serde(rename = "recordDate")]
  record_date: Option<DateTime<Utc>>, 
  #[serde(rename = "declaredDate")]
  declared_date: Option<DateTime<Utc>>, 
  #[serde(rename = "ratio")]
  ratio: f32,  // 0.142857 
  #[serde(rename = "tofactor")]
  tofactor: f32,  // 7.0 
  #[serde(rename = "forfactor")]
  forfactor: f32  // 1.0 
}

impl Split {
  pub fn new(ticker: TickerSymbol, ex_date: DateTime<Utc>, payment_date: DateTime<Utc>, ratio: f32, tofactor: f32, forfactor: f32, ) -> Split {
    Split {
      ticker: ticker,
      ex_date: ex_date,
      payment_date: payment_date,
      record_date: None,
      declared_date: None,
      ratio: ratio,
      tofactor: tofactor,
      forfactor: forfactor
    }
  }

  pub fn set_ticker(&mut self, ticker: TickerSymbol) {
    self.ticker = ticker;
  }

  pub fn with_ticker(mut self, ticker: TickerSymbol) -> Split {
    self.ticker = ticker;
    self
  }

  pub fn ticker(&self) -> &TickerSymbol {
    &self.ticker
  }


  pub fn set_ex_date(&mut self, ex_date: DateTime<Utc>) {
    self.ex_date = ex_date;
  }

  pub fn with_ex_date(mut self, ex_date: DateTime<Utc>) -> Split {
    self.ex_date = ex_date;
    self
  }

  pub fn ex_date(&self) -> &DateTime<Utc> {
    &self.ex_date
  }


  pub fn set_payment_date(&mut self, payment_date: DateTime<Utc>) {
    self.payment_date = payment_date;
  }

  pub fn with_payment_date(mut self, payment_date: DateTime<Utc>) -> Split {
    self.payment_date = payment_date;
    self
  }

  pub fn payment_date(&self) -> &DateTime<Utc> {
    &self.payment_date
  }


  pub fn set_record_date(&mut self, record_date: DateTime<Utc>) {
    self.record_date = Some(record_date);
  }

  pub fn with_record_date(mut self, record_date: DateTime<Utc>) -> Split {
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

  pub fn with_declared_date(mut self, declared_date: DateTime<Utc>) -> Split {
    self.declared_date = Some(declared_date);
    self
  }

  pub fn declared_date(&self) -> Option<&DateTime<Utc>> {
    self.declared_date.as_ref()
  }

  pub fn reset_declared_date(&mut self) {
    self.declared_date = None;
  }

  pub fn set_ratio(&mut self, ratio: f32) {
    self.ratio = ratio;
  }

  pub fn with_ratio(mut self, ratio: f32) -> Split {
    self.ratio = ratio;
    self
  }

  pub fn ratio(&self) -> &f32 {
    &self.ratio
  }


  pub fn set_tofactor(&mut self, tofactor: f32) {
    self.tofactor = tofactor;
  }

  pub fn with_tofactor(mut self, tofactor: f32) -> Split {
    self.tofactor = tofactor;
    self
  }

  pub fn tofactor(&self) -> &f32 {
    &self.tofactor
  }


  pub fn set_forfactor(&mut self, forfactor: f32) {
    self.forfactor = forfactor;
  }

  pub fn with_forfactor(mut self, forfactor: f32) -> Split {
    self.forfactor = forfactor;
    self
  }

  pub fn forfactor(&self) -> &f32 {
    &self.forfactor
  }


}


