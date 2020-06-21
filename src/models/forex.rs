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
pub struct Forex {
  #[serde(rename = "a")]
  a: i64, 
  #[serde(rename = "b")]
  b: i64, 
  #[serde(rename = "t")]
  t: i64  // 1517529600225 
}

impl Forex {
  pub fn new(a: i64, b: i64, t: i64, ) -> Forex {
    Forex {
      a: a,
      b: b,
      t: t
    }
  }

  pub fn set_a(&mut self, a: i64) {
    self.a = a;
  }

  pub fn with_a(mut self, a: i64) -> Forex {
    self.a = a;
    self
  }

  pub fn a(&self) -> &i64 {
    &self.a
  }


  pub fn set_b(&mut self, b: i64) {
    self.b = b;
  }

  pub fn with_b(mut self, b: i64) -> Forex {
    self.b = b;
    self
  }

  pub fn b(&self) -> &i64 {
    &self.b
  }


  pub fn set_t(&mut self, t: i64) {
    self.t = t;
  }

  pub fn with_t(mut self, t: i64) -> Forex {
    self.t = t;
    self
  }

  pub fn t(&self) -> &i64 {
    &self.t
  }


}


