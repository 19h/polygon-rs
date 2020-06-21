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
pub struct Trade {
  #[serde(rename = "c1")]
  c1: i64,  // 14 
  #[serde(rename = "c2")]
  c2: i64,  // 12 
  #[serde(rename = "c3")]
  c3: i64,  // 0 
  #[serde(rename = "c4")]
  c4: i64,  // 0 
  #[serde(rename = "e")]
  e: String,  // 12 
  #[serde(rename = "p")]
  p: i64, 
  #[serde(rename = "s")]
  s: i64,  // 50 
  #[serde(rename = "t")]
  t: i64  // 1517529601006 
}

impl Trade {
  pub fn new(c1: i64, c2: i64, c3: i64, c4: i64, e: String, p: i64, s: i64, t: i64, ) -> Trade {
    Trade {
      c1: c1,
      c2: c2,
      c3: c3,
      c4: c4,
      e: e,
      p: p,
      s: s,
      t: t
    }
  }

  pub fn set_c1(&mut self, c1: i64) {
    self.c1 = c1;
  }

  pub fn with_c1(mut self, c1: i64) -> Trade {
    self.c1 = c1;
    self
  }

  pub fn c1(&self) -> &i64 {
    &self.c1
  }


  pub fn set_c2(&mut self, c2: i64) {
    self.c2 = c2;
  }

  pub fn with_c2(mut self, c2: i64) -> Trade {
    self.c2 = c2;
    self
  }

  pub fn c2(&self) -> &i64 {
    &self.c2
  }


  pub fn set_c3(&mut self, c3: i64) {
    self.c3 = c3;
  }

  pub fn with_c3(mut self, c3: i64) -> Trade {
    self.c3 = c3;
    self
  }

  pub fn c3(&self) -> &i64 {
    &self.c3
  }


  pub fn set_c4(&mut self, c4: i64) {
    self.c4 = c4;
  }

  pub fn with_c4(mut self, c4: i64) -> Trade {
    self.c4 = c4;
    self
  }

  pub fn c4(&self) -> &i64 {
    &self.c4
  }


  pub fn set_e(&mut self, e: String) {
    self.e = e;
  }

  pub fn with_e(mut self, e: String) -> Trade {
    self.e = e;
    self
  }

  pub fn e(&self) -> &String {
    &self.e
  }


  pub fn set_p(&mut self, p: i64) {
    self.p = p;
  }

  pub fn with_p(mut self, p: i64) -> Trade {
    self.p = p;
    self
  }

  pub fn p(&self) -> &i64 {
    &self.p
  }


  pub fn set_s(&mut self, s: i64) {
    self.s = s;
  }

  pub fn with_s(mut self, s: i64) -> Trade {
    self.s = s;
    self
  }

  pub fn s(&self) -> &i64 {
    &self.s
  }


  pub fn set_t(&mut self, t: i64) {
    self.t = t;
  }

  pub fn with_t(mut self, t: i64) -> Trade {
    self.t = t;
    self
  }

  pub fn t(&self) -> &i64 {
    &self.t
  }


}


