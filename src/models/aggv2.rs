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
pub struct Aggv2 {
  #[serde(rename = "T")]
  T: Option<String>,  // AAPL 
  #[serde(rename = "v")]
  v: i64,  // 31315282 
  #[serde(rename = "o")]
  o: i64, 
  #[serde(rename = "c")]
  c: i64, 
  #[serde(rename = "h")]
  h: i64, 
  #[serde(rename = "l")]
  l: i64, 
  #[serde(rename = "t")]
  t: Option<f32>,  // 1.549314E+12 
  #[serde(rename = "n")]
  n: Option<f32>  // 4.0 
}

impl Aggv2 {
  pub fn new(v: i64, o: i64, c: i64, h: i64, l: i64, ) -> Aggv2 {
    Aggv2 {
      T: None,
      v: v,
      o: o,
      c: c,
      h: h,
      l: l,
      t: None,
      n: None
    }
  }

  pub fn set_T(&mut self, T: String) {
    self.T = Some(T);
  }

  pub fn with_T(mut self, T: String) -> Aggv2 {
    self.T = Some(T);
    self
  }

  pub fn T(&self) -> Option<&String> {
    self.T.as_ref()
  }

  pub fn reset_T(&mut self) {
    self.T = None;
  }

  pub fn set_v(&mut self, v: i64) {
    self.v = v;
  }

  pub fn with_v(mut self, v: i64) -> Aggv2 {
    self.v = v;
    self
  }

  pub fn v(&self) -> &i64 {
    &self.v
  }


  pub fn set_o(&mut self, o: i64) {
    self.o = o;
  }

  pub fn with_o(mut self, o: i64) -> Aggv2 {
    self.o = o;
    self
  }

  pub fn o(&self) -> &i64 {
    &self.o
  }


  pub fn set_c(&mut self, c: i64) {
    self.c = c;
  }

  pub fn with_c(mut self, c: i64) -> Aggv2 {
    self.c = c;
    self
  }

  pub fn c(&self) -> &i64 {
    &self.c
  }


  pub fn set_h(&mut self, h: i64) {
    self.h = h;
  }

  pub fn with_h(mut self, h: i64) -> Aggv2 {
    self.h = h;
    self
  }

  pub fn h(&self) -> &i64 {
    &self.h
  }


  pub fn set_l(&mut self, l: i64) {
    self.l = l;
  }

  pub fn with_l(mut self, l: i64) -> Aggv2 {
    self.l = l;
    self
  }

  pub fn l(&self) -> &i64 {
    &self.l
  }


  pub fn set_t(&mut self, t: f32) {
    self.t = Some(t);
  }

  pub fn with_t(mut self, t: f32) -> Aggv2 {
    self.t = Some(t);
    self
  }

  pub fn t(&self) -> Option<&f32> {
    self.t.as_ref()
  }

  pub fn reset_t(&mut self) {
    self.t = None;
  }

  pub fn set_n(&mut self, n: f32) {
    self.n = Some(n);
  }

  pub fn with_n(mut self, n: f32) -> Aggv2 {
    self.n = Some(n);
    self
  }

  pub fn n(&self) -> Option<&f32> {
    self.n.as_ref()
  }

  pub fn reset_n(&mut self) {
    self.n = None;
  }

}


