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
pub struct StocksV2Nbbo {
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
  #[serde(rename = "c")]
  c: Option<Vec<i64>>, 
  #[serde(rename = "i")]
  i: Option<Vec<i64>>, 
  #[serde(rename = "p")]
  p: i64, 
  #[serde(rename = "x")]
  x: i64,  // 11 
  #[serde(rename = "s")]
  s: i64,  // 100 
  #[serde(rename = "P")]
  P: i64, 
  #[serde(rename = "X")]
  X: i64,  // 11 
  #[serde(rename = "S")]
  S: i64,  // 100 
  #[serde(rename = "z")]
  z: i64  // 1 
}

impl StocksV2Nbbo {
  pub fn new(t: i64, q: i64, p: i64, x: i64, s: i64, P: i64, X: i64, S: i64, z: i64, ) -> StocksV2Nbbo {
    StocksV2Nbbo {
      T: None,
      t: t,
      y: None,
      f: None,
      q: q,
      c: None,
      i: None,
      p: p,
      x: x,
      s: s,
      P: P,
      X: X,
      S: S,
      z: z
    }
  }

  pub fn set_T(&mut self, T: String) {
    self.T = Some(T);
  }

  pub fn with_T(mut self, T: String) -> StocksV2Nbbo {
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

  pub fn with_t(mut self, t: i64) -> StocksV2Nbbo {
    self.t = t;
    self
  }

  pub fn t(&self) -> &i64 {
    &self.t
  }


  pub fn set_y(&mut self, y: i64) {
    self.y = Some(y);
  }

  pub fn with_y(mut self, y: i64) -> StocksV2Nbbo {
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

  pub fn with_f(mut self, f: i64) -> StocksV2Nbbo {
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

  pub fn with_q(mut self, q: i64) -> StocksV2Nbbo {
    self.q = q;
    self
  }

  pub fn q(&self) -> &i64 {
    &self.q
  }


  pub fn set_c(&mut self, c: Vec<i64>) {
    self.c = Some(c);
  }

  pub fn with_c(mut self, c: Vec<i64>) -> StocksV2Nbbo {
    self.c = Some(c);
    self
  }

  pub fn c(&self) -> Option<&Vec<i64>> {
    self.c.as_ref()
  }

  pub fn reset_c(&mut self) {
    self.c = None;
  }

  pub fn set_i(&mut self, i: Vec<i64>) {
    self.i = Some(i);
  }

  pub fn with_i(mut self, i: Vec<i64>) -> StocksV2Nbbo {
    self.i = Some(i);
    self
  }

  pub fn i(&self) -> Option<&Vec<i64>> {
    self.i.as_ref()
  }

  pub fn reset_i(&mut self) {
    self.i = None;
  }

  pub fn set_p(&mut self, p: i64) {
    self.p = p;
  }

  pub fn with_p(mut self, p: i64) -> StocksV2Nbbo {
    self.p = p;
    self
  }

  pub fn p(&self) -> &i64 {
    &self.p
  }


  pub fn set_x(&mut self, x: i64) {
    self.x = x;
  }

  pub fn with_x(mut self, x: i64) -> StocksV2Nbbo {
    self.x = x;
    self
  }

  pub fn x(&self) -> &i64 {
    &self.x
  }


  pub fn set_s(&mut self, s: i64) {
    self.s = s;
  }

  pub fn with_s(mut self, s: i64) -> StocksV2Nbbo {
    self.s = s;
    self
  }

  pub fn s(&self) -> &i64 {
    &self.s
  }


  pub fn set_P(&mut self, P: i64) {
    self.P = P;
  }

  pub fn with_P(mut self, P: i64) -> StocksV2Nbbo {
    self.P = P;
    self
  }

  pub fn P(&self) -> &i64 {
    &self.P
  }


  pub fn set_X(&mut self, X: i64) {
    self.X = X;
  }

  pub fn with_X(mut self, X: i64) -> StocksV2Nbbo {
    self.X = X;
    self
  }

  pub fn X(&self) -> &i64 {
    &self.X
  }


  pub fn set_S(&mut self, S: i64) {
    self.S = S;
  }

  pub fn with_S(mut self, S: i64) -> StocksV2Nbbo {
    self.S = S;
    self
  }

  pub fn S(&self) -> &i64 {
    &self.S
  }


  pub fn set_z(&mut self, z: i64) {
    self.z = z;
  }

  pub fn with_z(mut self, z: i64) -> StocksV2Nbbo {
    self.z = z;
    self
  }

  pub fn z(&self) -> &i64 {
    &self.z
  }


}


