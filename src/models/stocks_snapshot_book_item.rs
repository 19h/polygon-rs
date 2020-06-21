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
pub struct StocksSnapshotBookItem {
  #[serde(rename = "p")]
  p: i64, 
  #[serde(rename = "x")]
  x: Value  // {"2":0.553,"5":2.32} 
}

impl StocksSnapshotBookItem {
  pub fn new(p: i64, x: Value, ) -> StocksSnapshotBookItem {
    StocksSnapshotBookItem {
      p: p,
      x: x
    }
  }

  pub fn set_p(&mut self, p: i64) {
    self.p = p;
  }

  pub fn with_p(mut self, p: i64) -> StocksSnapshotBookItem {
    self.p = p;
    self
  }

  pub fn p(&self) -> &i64 {
    &self.p
  }


  pub fn set_x(&mut self, x: Value) {
    self.x = x;
  }

  pub fn with_x(mut self, x: Value) -> StocksSnapshotBookItem {
    self.x = x;
    self
  }

  pub fn x(&self) -> &Value {
    &self.x
  }


}


