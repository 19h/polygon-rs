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
pub struct Quote {
  #[serde(rename = "c")]
  c: i64,  // 0 
  #[serde(rename = "bE")]
  b_e: String,  // 11 
  #[serde(rename = "aE")]
  a_e: String,  // 12 
  #[serde(rename = "aP")]
  a_p: i64, 
  #[serde(rename = "bP")]
  b_p: i64, 
  #[serde(rename = "bS")]
  b_s: i64,  // 25 
  #[serde(rename = "aS")]
  a_s: i64,  // 55 
  #[serde(rename = "t")]
  t: i64  // 1517529601006 
}

impl Quote {
  pub fn new(c: i64, b_e: String, a_e: String, a_p: i64, b_p: i64, b_s: i64, a_s: i64, t: i64, ) -> Quote {
    Quote {
      c: c,
      b_e: b_e,
      a_e: a_e,
      a_p: a_p,
      b_p: b_p,
      b_s: b_s,
      a_s: a_s,
      t: t
    }
  }

  pub fn set_c(&mut self, c: i64) {
    self.c = c;
  }

  pub fn with_c(mut self, c: i64) -> Quote {
    self.c = c;
    self
  }

  pub fn c(&self) -> &i64 {
    &self.c
  }


  pub fn set_b_e(&mut self, b_e: String) {
    self.b_e = b_e;
  }

  pub fn with_b_e(mut self, b_e: String) -> Quote {
    self.b_e = b_e;
    self
  }

  pub fn b_e(&self) -> &String {
    &self.b_e
  }


  pub fn set_a_e(&mut self, a_e: String) {
    self.a_e = a_e;
  }

  pub fn with_a_e(mut self, a_e: String) -> Quote {
    self.a_e = a_e;
    self
  }

  pub fn a_e(&self) -> &String {
    &self.a_e
  }


  pub fn set_a_p(&mut self, a_p: i64) {
    self.a_p = a_p;
  }

  pub fn with_a_p(mut self, a_p: i64) -> Quote {
    self.a_p = a_p;
    self
  }

  pub fn a_p(&self) -> &i64 {
    &self.a_p
  }


  pub fn set_b_p(&mut self, b_p: i64) {
    self.b_p = b_p;
  }

  pub fn with_b_p(mut self, b_p: i64) -> Quote {
    self.b_p = b_p;
    self
  }

  pub fn b_p(&self) -> &i64 {
    &self.b_p
  }


  pub fn set_b_s(&mut self, b_s: i64) {
    self.b_s = b_s;
  }

  pub fn with_b_s(mut self, b_s: i64) -> Quote {
    self.b_s = b_s;
    self
  }

  pub fn b_s(&self) -> &i64 {
    &self.b_s
  }


  pub fn set_a_s(&mut self, a_s: i64) {
    self.a_s = a_s;
  }

  pub fn with_a_s(mut self, a_s: i64) -> Quote {
    self.a_s = a_s;
    self
  }

  pub fn a_s(&self) -> &i64 {
    &self.a_s
  }


  pub fn set_t(&mut self, t: i64) {
    self.t = t;
  }

  pub fn with_t(mut self, t: i64) -> Quote {
    self.t = t;
    self
  }

  pub fn t(&self) -> &i64 {
    &self.t
  }


}


