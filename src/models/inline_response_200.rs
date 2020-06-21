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
pub struct InlineResponse200 {
  #[serde(rename = "status")]
  status: Option<String>,  // OK 
  #[serde(rename = "results")]
  results: Option<Value>  // {"types":{"CS":"Common Stock","ADR":"American Depository Receipt","NVDR":"Non-Voting Depository Receipt","GDR":"Global Depositary Receipt"},"indexTypes":{"INDEX":"Index","ETF":"Exchange Traded Fund (ETF)","ETN":"Exchange Traded Note (ETN)"}} 
}

impl InlineResponse200 {
  pub fn new() -> InlineResponse200 {
    InlineResponse200 {
      status: None,
      results: None
    }
  }

  pub fn set_status(&mut self, status: String) {
    self.status = Some(status);
  }

  pub fn with_status(mut self, status: String) -> InlineResponse200 {
    self.status = Some(status);
    self
  }

  pub fn status(&self) -> Option<&String> {
    self.status.as_ref()
  }

  pub fn reset_status(&mut self) {
    self.status = None;
  }

  pub fn set_results(&mut self, results: Value) {
    self.results = Some(results);
  }

  pub fn with_results(mut self, results: Value) -> InlineResponse200 {
    self.results = Some(results);
    self
  }

  pub fn results(&self) -> Option<&Value> {
    self.results.as_ref()
  }

  pub fn reset_results(&mut self) {
    self.results = None;
  }

}


