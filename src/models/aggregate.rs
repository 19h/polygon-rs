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
pub struct Aggregate {
  #[serde(rename = "o")]
  o: i64, 
  #[serde(rename = "c")]
  c: i64, 
  #[serde(rename = "l")]
  l: i64, 
  #[serde(rename = "h")]
  h: i64, 
  #[serde(rename = "v")]
  v: i64,  // 1800 
  #[serde(rename = "k")]
  k: i64,  // 4 
  #[serde(rename = "t")]
  t: i64  // 1517529605000 
}

impl Aggregate {
  pub fn new(o: i64, c: i64, l: i64, h: i64, v: i64, k: i64, t: i64, ) -> Aggregate {
    Aggregate {
      o: o,
      c: c,
      l: l,
      h: h,
      v: v,
      k: k,
      t: t
    }
  }

  pub fn set_o(&mut self, o: i64) {
    self.o = o;
  }

  pub fn with_o(mut self, o: i64) -> Aggregate {
    self.o = o;
    self
  }

  pub fn o(&self) -> &i64 {
    &self.o
  }


  pub fn set_c(&mut self, c: i64) {
    self.c = c;
  }

  pub fn with_c(mut self, c: i64) -> Aggregate {
    self.c = c;
    self
  }

  pub fn c(&self) -> &i64 {
    &self.c
  }


  pub fn set_l(&mut self, l: i64) {
    self.l = l;
  }

  pub fn with_l(mut self, l: i64) -> Aggregate {
    self.l = l;
    self
  }

  pub fn l(&self) -> &i64 {
    &self.l
  }


  pub fn set_h(&mut self, h: i64) {
    self.h = h;
  }

  pub fn with_h(mut self, h: i64) -> Aggregate {
    self.h = h;
    self
  }

  pub fn h(&self) -> &i64 {
    &self.h
  }


  pub fn set_v(&mut self, v: i64) {
    self.v = v;
  }

  pub fn with_v(mut self, v: i64) -> Aggregate {
    self.v = v;
    self
  }

  pub fn v(&self) -> &i64 {
    &self.v
  }


  pub fn set_k(&mut self, k: i64) {
    self.k = k;
  }

  pub fn with_k(mut self, k: i64) -> Aggregate {
    self.k = k;
    self
  }

  pub fn k(&self) -> &i64 {
    &self.k
  }


  pub fn set_t(&mut self, t: i64) {
    self.t = t;
  }

  pub fn with_t(mut self, t: i64) -> Aggregate {
    self.t = t;
    self
  }

  pub fn t(&self) -> &i64 {
    &self.t
  }


}


