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
pub struct InlineResponse20017LastAverage {
  #[serde(rename = "avg")]
  avg: Option<f32>,  // 9348.727 
  #[serde(rename = "tradesAveraged")]
  trades_averaged: Option<i64>  // 10 
}

impl InlineResponse20017LastAverage {
  pub fn new() -> InlineResponse20017LastAverage {
    InlineResponse20017LastAverage {
      avg: None,
      trades_averaged: None
    }
  }

  pub fn set_avg(&mut self, avg: f32) {
    self.avg = Some(avg);
  }

  pub fn with_avg(mut self, avg: f32) -> InlineResponse20017LastAverage {
    self.avg = Some(avg);
    self
  }

  pub fn avg(&self) -> Option<&f32> {
    self.avg.as_ref()
  }

  pub fn reset_avg(&mut self) {
    self.avg = None;
  }

  pub fn set_trades_averaged(&mut self, trades_averaged: i64) {
    self.trades_averaged = Some(trades_averaged);
  }

  pub fn with_trades_averaged(mut self, trades_averaged: i64) -> InlineResponse20017LastAverage {
    self.trades_averaged = Some(trades_averaged);
    self
  }

  pub fn trades_averaged(&self) -> Option<&i64> {
    self.trades_averaged.as_ref()
  }

  pub fn reset_trades_averaged(&mut self) {
    self.trades_averaged = None;
  }

}

