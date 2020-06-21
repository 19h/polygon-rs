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
pub struct ForexAggregate {
  #[serde(rename = "o")]
  o: i64, 
  #[serde(rename = "c")]
  c: i64, 
  #[serde(rename = "l")]
  l: i64, 
  #[serde(rename = "h")]
  h: i64, 
  #[serde(rename = "v")]
  v: i64,  // 3988 
  #[serde(rename = "t")]
  t: i64  // 1517529605000 
}

impl ForexAggregate {
  pub fn new(o: i64, c: i64, l: i64, h: i64, v: i64, t: i64, ) -> ForexAggregate {
    ForexAggregate {
      o: o,
      c: c,
      l: l,
      h: h,
      v: v,
      t: t
    }
  }

  pub fn set_o(&mut self, o: i64) {
    self.o = o;
  }

  pub fn with_o(mut self, o: i64) -> ForexAggregate {
    self.o = o;
    self
  }

  pub fn o(&self) -> &i64 {
    &self.o
  }


  pub fn set_c(&mut self, c: i64) {
    self.c = c;
  }

  pub fn with_c(mut self, c: i64) -> ForexAggregate {
    self.c = c;
    self
  }

  pub fn c(&self) -> &i64 {
    &self.c
  }


  pub fn set_l(&mut self, l: i64) {
    self.l = l;
  }

  pub fn with_l(mut self, l: i64) -> ForexAggregate {
    self.l = l;
    self
  }

  pub fn l(&self) -> &i64 {
    &self.l
  }


  pub fn set_h(&mut self, h: i64) {
    self.h = h;
  }

  pub fn with_h(mut self, h: i64) -> ForexAggregate {
    self.h = h;
    self
  }

  pub fn h(&self) -> &i64 {
    &self.h
  }


  pub fn set_v(&mut self, v: i64) {
    self.v = v;
  }

  pub fn with_v(mut self, v: i64) -> ForexAggregate {
    self.v = v;
    self
  }

  pub fn v(&self) -> &i64 {
    &self.v
  }


  pub fn set_t(&mut self, t: i64) {
    self.t = t;
  }

  pub fn with_t(mut self, t: i64) -> ForexAggregate {
    self.t = t;
    self
  }

  pub fn t(&self) -> &i64 {
    &self.t
  }


}


