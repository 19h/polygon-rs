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
pub struct InlineResponse2004 {
  #[serde(rename = "status")]
  status: Option<String>,  // OK 
  #[serde(rename = "count")]
  count: Option<i64>,  // 1 
  #[serde(rename = "results")]
  results: Option<Vec<Split>> 
}

impl InlineResponse2004 {
  pub fn new() -> InlineResponse2004 {
    InlineResponse2004 {
      status: None,
      count: None,
      results: None
    }
  }

  pub fn set_status(&mut self, status: String) {
    self.status = Some(status);
  }

  pub fn with_status(mut self, status: String) -> InlineResponse2004 {
    self.status = Some(status);
    self
  }

  pub fn status(&self) -> Option<&String> {
    self.status.as_ref()
  }

  pub fn reset_status(&mut self) {
    self.status = None;
  }

  pub fn set_count(&mut self, count: i64) {
    self.count = Some(count);
  }

  pub fn with_count(mut self, count: i64) -> InlineResponse2004 {
    self.count = Some(count);
    self
  }

  pub fn count(&self) -> Option<&i64> {
    self.count.as_ref()
  }

  pub fn reset_count(&mut self) {
    self.count = None;
  }

  pub fn set_results(&mut self, results: Vec<Split>) {
    self.results = Some(results);
  }

  pub fn with_results(mut self, results: Vec<Split>) -> InlineResponse2004 {
    self.results = Some(results);
    self
  }

  pub fn results(&self) -> Option<&Vec<Split>> {
    self.results.as_ref()
  }

  pub fn reset_results(&mut self) {
    self.results = None;
  }

}


