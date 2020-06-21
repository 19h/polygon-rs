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
pub struct CryptoTickJson {
  #[serde(rename = "p")]
  p: i64, 
  #[serde(rename = "s")]
  s: i64, 
  #[serde(rename = "x")]
  x: i64,  // 2 
  #[serde(rename = "c")]
  c: Vec<i64>,  // [2,0] 
  #[serde(rename = "t")]
  t: i64  // 1525930460102 
}

impl CryptoTickJson {
  pub fn new(p: i64, s: i64, x: i64, c: Vec<i64>, t: i64, ) -> CryptoTickJson {
    CryptoTickJson {
      p: p,
      s: s,
      x: x,
      c: c,
      t: t
    }
  }

  pub fn set_p(&mut self, p: i64) {
    self.p = p;
  }

  pub fn with_p(mut self, p: i64) -> CryptoTickJson {
    self.p = p;
    self
  }

  pub fn p(&self) -> &i64 {
    &self.p
  }


  pub fn set_s(&mut self, s: i64) {
    self.s = s;
  }

  pub fn with_s(mut self, s: i64) -> CryptoTickJson {
    self.s = s;
    self
  }

  pub fn s(&self) -> &i64 {
    &self.s
  }


  pub fn set_x(&mut self, x: i64) {
    self.x = x;
  }

  pub fn with_x(mut self, x: i64) -> CryptoTickJson {
    self.x = x;
    self
  }

  pub fn x(&self) -> &i64 {
    &self.x
  }


  pub fn set_c(&mut self, c: Vec<i64>) {
    self.c = c;
  }

  pub fn with_c(mut self, c: Vec<i64>) -> CryptoTickJson {
    self.c = c;
    self
  }

  pub fn c(&self) -> &Vec<i64> {
    &self.c
  }


  pub fn set_t(&mut self, t: i64) {
    self.t = t;
  }

  pub fn with_t(mut self, t: i64) -> CryptoTickJson {
    self.t = t;
    self
  }

  pub fn t(&self) -> &i64 {
    &self.t
  }


}


