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
pub struct InlineResponse20011 {
  #[serde(rename = "day")]
  day: String,  // Fri Feb 02 00:00:00 GMT 2018 
  #[serde(rename = "map")]
  map: Value,  // {"a":"ask","b":"bid","t":"timestamp"} 
  #[serde(rename = "msLatency")]
  ms_latency: i64,  // 2 
  #[serde(rename = "status")]
  status: String,  // success 
  #[serde(rename = "pair")]
  pair: String,  // AUD/USD 
  #[serde(rename = "ticks")]
  ticks: Vec<Forex> 
}

impl InlineResponse20011 {
  pub fn new(day: String, map: Value, ms_latency: i64, status: String, pair: String, ticks: Vec<Forex>, ) -> InlineResponse20011 {
    InlineResponse20011 {
      day: day,
      map: map,
      ms_latency: ms_latency,
      status: status,
      pair: pair,
      ticks: ticks
    }
  }

  pub fn set_day(&mut self, day: String) {
    self.day = day;
  }

  pub fn with_day(mut self, day: String) -> InlineResponse20011 {
    self.day = day;
    self
  }

  pub fn day(&self) -> &String {
    &self.day
  }


  pub fn set_map(&mut self, map: Value) {
    self.map = map;
  }

  pub fn with_map(mut self, map: Value) -> InlineResponse20011 {
    self.map = map;
    self
  }

  pub fn map(&self) -> &Value {
    &self.map
  }


  pub fn set_ms_latency(&mut self, ms_latency: i64) {
    self.ms_latency = ms_latency;
  }

  pub fn with_ms_latency(mut self, ms_latency: i64) -> InlineResponse20011 {
    self.ms_latency = ms_latency;
    self
  }

  pub fn ms_latency(&self) -> &i64 {
    &self.ms_latency
  }


  pub fn set_status(&mut self, status: String) {
    self.status = status;
  }

  pub fn with_status(mut self, status: String) -> InlineResponse20011 {
    self.status = status;
    self
  }

  pub fn status(&self) -> &String {
    &self.status
  }


  pub fn set_pair(&mut self, pair: String) {
    self.pair = pair;
  }

  pub fn with_pair(mut self, pair: String) -> InlineResponse20011 {
    self.pair = pair;
    self
  }

  pub fn pair(&self) -> &String {
    &self.pair
  }


  pub fn set_ticks(&mut self, ticks: Vec<Forex>) {
    self.ticks = ticks;
  }

  pub fn with_ticks(mut self, ticks: Vec<Forex>) -> InlineResponse20011 {
    self.ticks = ticks;
    self
  }

  pub fn ticks(&self) -> &Vec<Forex> {
    &self.ticks
  }


}


