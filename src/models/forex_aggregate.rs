/* 
 * Polygon API
 *
 * The future of fintech.
 *
 * OpenAPI spec version: 1.0.1
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
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
  o: f32,  // 0.81151 
  #[serde(rename = "c")]
  c: f32,  // 0.81287 
  #[serde(rename = "l")]
  l: f32,  // 0.81 
  #[serde(rename = "h")]
  h: f32,  // 0.8141 
  #[serde(rename = "v")]
  v: i64,  // 3988 
  #[serde(rename = "t")]
  t: i64  // 1517529605000 
}

impl ForexAggregate {
  pub fn new(o: f32, c: f32, l: f32, h: f32, v: i64, t: i64, ) -> ForexAggregate {
    ForexAggregate {
      o: o,
      c: c,
      l: l,
      h: h,
      v: v,
      t: t
    }
  }

  pub fn set_o(&mut self, o: f32) {
    self.o = o;
  }

  pub fn with_o(mut self, o: f32) -> ForexAggregate {
    self.o = o;
    self
  }

  pub fn o(&self) -> &f32 {
    &self.o
  }


  pub fn set_c(&mut self, c: f32) {
    self.c = c;
  }

  pub fn with_c(mut self, c: f32) -> ForexAggregate {
    self.c = c;
    self
  }

  pub fn c(&self) -> &f32 {
    &self.c
  }


  pub fn set_l(&mut self, l: f32) {
    self.l = l;
  }

  pub fn with_l(mut self, l: f32) -> ForexAggregate {
    self.l = l;
    self
  }

  pub fn l(&self) -> &f32 {
    &self.l
  }


  pub fn set_h(&mut self, h: f32) {
    self.h = h;
  }

  pub fn with_h(mut self, h: f32) -> ForexAggregate {
    self.h = h;
    self
  }

  pub fn h(&self) -> &f32 {
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


