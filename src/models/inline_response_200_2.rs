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
pub struct InlineResponse2002 {
  #[serde(rename = "status")]
  status: Option<String>,  // OK 
  #[serde(rename = "results")]
  results: Option<Vec<String>>  // [{"locale":"US","name":"United States"},{"locale":"GB","name":"Great Britain / UK"},{"locale":"CA","name":"Canada"}] 
}

impl InlineResponse2002 {
  pub fn new() -> InlineResponse2002 {
    InlineResponse2002 {
      status: None,
      results: None
    }
  }

  pub fn set_status(&mut self, status: String) {
    self.status = Some(status);
  }

  pub fn with_status(mut self, status: String) -> InlineResponse2002 {
    self.status = Some(status);
    self
  }

  pub fn status(&self) -> Option<&String> {
    self.status.as_ref()
  }

  pub fn reset_status(&mut self) {
    self.status = None;
  }

  pub fn set_results(&mut self, results: Vec<String>) {
    self.results = Some(results);
  }

  pub fn with_results(mut self, results: Vec<String>) -> InlineResponse2002 {
    self.results = Some(results);
    self
  }

  pub fn results(&self) -> Option<&Vec<String>> {
    self.results.as_ref()
  }

  pub fn reset_results(&mut self) {
    self.results = None;
  }

}


