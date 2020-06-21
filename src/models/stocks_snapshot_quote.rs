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
pub struct StocksSnapshotQuote {
  #[serde(rename = "p")]
  p: i64,  // 120 
  #[serde(rename = "s")]
  s: i64,  // 5 
  #[serde(rename = "P")]
  P: i64,  // 121 
  #[serde(rename = "S")]
  S: i64,  // 3 
  #[serde(rename = "t")]
  t: i64  // 1547787608999000000 
}

impl StocksSnapshotQuote {
  pub fn new(p: i64, s: i64, P: i64, S: i64, t: i64, ) -> StocksSnapshotQuote {
    StocksSnapshotQuote {
      p: p,
      s: s,
      P: P,
      S: S,
      t: t
    }
  }

  pub fn set_p(&mut self, p: i64) {
    self.p = p;
  }

  pub fn with_p(mut self, p: i64) -> StocksSnapshotQuote {
    self.p = p;
    self
  }

  pub fn p(&self) -> &i64 {
    &self.p
  }


  pub fn set_s(&mut self, s: i64) {
    self.s = s;
  }

  pub fn with_s(mut self, s: i64) -> StocksSnapshotQuote {
    self.s = s;
    self
  }

  pub fn s(&self) -> &i64 {
    &self.s
  }


  pub fn set_P(&mut self, P: i64) {
    self.P = P;
  }

  pub fn with_P(mut self, P: i64) -> StocksSnapshotQuote {
    self.P = P;
    self
  }

  pub fn P(&self) -> &i64 {
    &self.P
  }


  pub fn set_S(&mut self, S: i64) {
    self.S = S;
  }

  pub fn with_S(mut self, S: i64) -> StocksSnapshotQuote {
    self.S = S;
    self
  }

  pub fn S(&self) -> &i64 {
    &self.S
  }


  pub fn set_t(&mut self, t: i64) {
    self.t = t;
  }

  pub fn with_t(mut self, t: i64) -> StocksSnapshotQuote {
    self.t = t;
    self
  }

  pub fn t(&self) -> &i64 {
    &self.t
  }


}


