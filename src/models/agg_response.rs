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
pub struct AggResponse {
  #[serde(rename = "ticker")]
  ticker: Option<String>,  // AAPL 
  #[serde(rename = "status")]
  status: String,  // OK 
  #[serde(rename = "adjusted")]
  adjusted: bool,  // true 
  #[serde(rename = "queryCount")]
  query_count: Option<i64>,  // 55 
  #[serde(rename = "resultsCount")]
  results_count: Option<i64>,  // 2 
  #[serde(rename = "results")]
  results: Vec<Aggv2> 
}

impl AggResponse {
  pub fn new(status: String, adjusted: bool, results: Vec<Aggv2>, ) -> AggResponse {
    AggResponse {
      ticker: None,
      status: status,
      adjusted: adjusted,
      query_count: None,
      results_count: None,
      results: results
    }
  }

  pub fn set_ticker(&mut self, ticker: String) {
    self.ticker = Some(ticker);
  }

  pub fn with_ticker(mut self, ticker: String) -> AggResponse {
    self.ticker = Some(ticker);
    self
  }

  pub fn ticker(&self) -> Option<&String> {
    self.ticker.as_ref()
  }

  pub fn reset_ticker(&mut self) {
    self.ticker = None;
  }

  pub fn set_status(&mut self, status: String) {
    self.status = status;
  }

  pub fn with_status(mut self, status: String) -> AggResponse {
    self.status = status;
    self
  }

  pub fn status(&self) -> &String {
    &self.status
  }


  pub fn set_adjusted(&mut self, adjusted: bool) {
    self.adjusted = adjusted;
  }

  pub fn with_adjusted(mut self, adjusted: bool) -> AggResponse {
    self.adjusted = adjusted;
    self
  }

  pub fn adjusted(&self) -> &bool {
    &self.adjusted
  }


  pub fn set_query_count(&mut self, query_count: i64) {
    self.query_count = Some(query_count);
  }

  pub fn with_query_count(mut self, query_count: i64) -> AggResponse {
    self.query_count = Some(query_count);
    self
  }

  pub fn query_count(&self) -> Option<&i64> {
    self.query_count.as_ref()
  }

  pub fn reset_query_count(&mut self) {
    self.query_count = None;
  }

  pub fn set_results_count(&mut self, results_count: i64) {
    self.results_count = Some(results_count);
  }

  pub fn with_results_count(mut self, results_count: i64) -> AggResponse {
    self.results_count = Some(results_count);
    self
  }

  pub fn results_count(&self) -> Option<&i64> {
    self.results_count.as_ref()
  }

  pub fn reset_results_count(&mut self) {
    self.results_count = None;
  }

  pub fn set_results(&mut self, results: Vec<Aggv2>) {
    self.results = results;
  }

  pub fn with_results(mut self, results: Vec<Aggv2>) -> AggResponse {
    self.results = results;
    self
  }

  pub fn results(&self) -> &Vec<Aggv2> {
    &self.results
  }


}


