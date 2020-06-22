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
pub struct InlineResponse2007 {
  #[serde(rename = "results_count")]
  results_count: Option<i64>,  // 10 
  #[serde(rename = "db_latency")]
  db_latency: Option<i64>,  // 2 
  #[serde(rename = "success")]
  success: bool,  // true 
  #[serde(rename = "ticker")]
  ticker: String,  // AAPL 
  #[serde(rename = "results")]
  results: Option<Vec<StocksV2Nbbo>> 
}

impl InlineResponse2007 {
  pub fn new(success: bool, ticker: String, ) -> InlineResponse2007 {
    InlineResponse2007 {
      results_count: None,
      db_latency: None,
      success: success,
      ticker: ticker,
      results: None
    }
  }

  pub fn set_results_count(&mut self, results_count: i64) {
    self.results_count = Some(results_count);
  }

  pub fn with_results_count(mut self, results_count: i64) -> InlineResponse2007 {
    self.results_count = Some(results_count);
    self
  }

  pub fn results_count(&self) -> Option<&i64> {
    self.results_count.as_ref()
  }

  pub fn reset_results_count(&mut self) {
    self.results_count = None;
  }

  pub fn set_db_latency(&mut self, db_latency: i64) {
    self.db_latency = Some(db_latency);
  }

  pub fn with_db_latency(mut self, db_latency: i64) -> InlineResponse2007 {
    self.db_latency = Some(db_latency);
    self
  }

  pub fn db_latency(&self) -> Option<&i64> {
    self.db_latency.as_ref()
  }

  pub fn reset_db_latency(&mut self) {
    self.db_latency = None;
  }

  pub fn set_success(&mut self, success: bool) {
    self.success = success;
  }

  pub fn with_success(mut self, success: bool) -> InlineResponse2007 {
    self.success = success;
    self
  }

  pub fn success(&self) -> &bool {
    &self.success
  }


  pub fn set_ticker(&mut self, ticker: String) {
    self.ticker = ticker;
  }

  pub fn with_ticker(mut self, ticker: String) -> InlineResponse2007 {
    self.ticker = ticker;
    self
  }

  pub fn ticker(&self) -> &String {
    &self.ticker
  }


  pub fn set_results(&mut self, results: Vec<StocksV2Nbbo>) {
    self.results = Some(results);
  }

  pub fn with_results(mut self, results: Vec<StocksV2Nbbo>) -> InlineResponse2007 {
    self.results = Some(results);
    self
  }

  pub fn results(&self) -> Option<&Vec<StocksV2Nbbo>> {
    self.results.as_ref()
  }

  pub fn reset_results(&mut self) {
    self.results = None;
  }

}


