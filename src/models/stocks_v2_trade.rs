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
pub struct StocksV2Trade {
  #[serde(rename = "T")]
  T: Option<String>,  // AAPL 
  #[serde(rename = "t")]
  t: i64,  // 1547787608999125800 
  #[serde(rename = "y")]
  y: Option<i64>,  // 1547787608999125800 
  #[serde(rename = "f")]
  f: Option<i64>,  // 1547787608999125800 
  #[serde(rename = "q")]
  q: i64,  // 23547 
  #[serde(rename = "i")]
  i: String,  // 00MGON 
  #[serde(rename = "x")]
  x: i64,  // 11 
  #[serde(rename = "s")]
  s: i64,  // 100 
  #[serde(rename = "c")]
  c: Vec<i64>, 
  #[serde(rename = "p")]
  p: i64, 
  #[serde(rename = "z")]
  z: i64  // 1 
}

impl StocksV2Trade {
  pub fn new(t: i64, q: i64, i: String, x: i64, s: i64, c: Vec<i64>, p: i64, z: i64, ) -> StocksV2Trade {
    StocksV2Trade {
      T: None,
      t: t,
      y: None,
      f: None,
      q: q,
      i: i,
      x: x,
      s: s,
      c: c,
      p: p,
      z: z
    }
  }

  pub fn set_T(&mut self, T: String) {
    self.T = Some(T);
  }

  pub fn with_T(mut self, T: String) -> StocksV2Trade {
    self.T = Some(T);
    self
  }

  pub fn T(&self) -> Option<&String> {
    self.T.as_ref()
  }

  pub fn reset_T(&mut self) {
    self.T = None;
  }

  pub fn set_t(&mut self, t: i64) {
    self.t = t;
  }

  pub fn with_t(mut self, t: i64) -> StocksV2Trade {
    self.t = t;
    self
  }

  pub fn t(&self) -> &i64 {
    &self.t
  }


  pub fn set_y(&mut self, y: i64) {
    self.y = Some(y);
  }

  pub fn with_y(mut self, y: i64) -> StocksV2Trade {
    self.y = Some(y);
    self
  }

  pub fn y(&self) -> Option<&i64> {
    self.y.as_ref()
  }

  pub fn reset_y(&mut self) {
    self.y = None;
  }

  pub fn set_f(&mut self, f: i64) {
    self.f = Some(f);
  }

  pub fn with_f(mut self, f: i64) -> StocksV2Trade {
    self.f = Some(f);
    self
  }

  pub fn f(&self) -> Option<&i64> {
    self.f.as_ref()
  }

  pub fn reset_f(&mut self) {
    self.f = None;
  }

  pub fn set_q(&mut self, q: i64) {
    self.q = q;
  }

  pub fn with_q(mut self, q: i64) -> StocksV2Trade {
    self.q = q;
    self
  }

  pub fn q(&self) -> &i64 {
    &self.q
  }


  pub fn set_i(&mut self, i: String) {
    self.i = i;
  }

  pub fn with_i(mut self, i: String) -> StocksV2Trade {
    self.i = i;
    self
  }

  pub fn i(&self) -> &String {
    &self.i
  }


  pub fn set_x(&mut self, x: i64) {
    self.x = x;
  }

  pub fn with_x(mut self, x: i64) -> StocksV2Trade {
    self.x = x;
    self
  }

  pub fn x(&self) -> &i64 {
    &self.x
  }


  pub fn set_s(&mut self, s: i64) {
    self.s = s;
  }

  pub fn with_s(mut self, s: i64) -> StocksV2Trade {
    self.s = s;
    self
  }

  pub fn s(&self) -> &i64 {
    &self.s
  }


  pub fn set_c(&mut self, c: Vec<i64>) {
    self.c = c;
  }

  pub fn with_c(mut self, c: Vec<i64>) -> StocksV2Trade {
    self.c = c;
    self
  }

  pub fn c(&self) -> &Vec<i64> {
    &self.c
  }


  pub fn set_p(&mut self, p: i64) {
    self.p = p;
  }

  pub fn with_p(mut self, p: i64) -> StocksV2Trade {
    self.p = p;
    self
  }

  pub fn p(&self) -> &i64 {
    &self.p
  }


  pub fn set_z(&mut self, z: i64) {
    self.z = z;
  }

  pub fn with_z(mut self, z: i64) -> StocksV2Trade {
    self.z = z;
    self
  }

  pub fn z(&self) -> &i64 {
    &self.z
  }


}


