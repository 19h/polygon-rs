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
pub struct LastQuote {
  #[serde(rename = "askprice")]
  askprice: i64, 
  #[serde(rename = "asksize")]
  asksize: i64,  // 2 
  #[serde(rename = "askexchange")]
  askexchange: i64,  // 11 
  #[serde(rename = "bidprice")]
  bidprice: i64, 
  #[serde(rename = "bidsize")]
  bidsize: i64,  // 20 
  #[serde(rename = "bidexchange")]
  bidexchange: i64,  // 12 
  #[serde(rename = "timestamp")]
  timestamp: i64  // 1518086601843 
}

impl LastQuote {
  pub fn new(askprice: i64, asksize: i64, askexchange: i64, bidprice: i64, bidsize: i64, bidexchange: i64, timestamp: i64, ) -> LastQuote {
    LastQuote {
      askprice: askprice,
      asksize: asksize,
      askexchange: askexchange,
      bidprice: bidprice,
      bidsize: bidsize,
      bidexchange: bidexchange,
      timestamp: timestamp
    }
  }

  pub fn set_askprice(&mut self, askprice: i64) {
    self.askprice = askprice;
  }

  pub fn with_askprice(mut self, askprice: i64) -> LastQuote {
    self.askprice = askprice;
    self
  }

  pub fn askprice(&self) -> &i64 {
    &self.askprice
  }


  pub fn set_asksize(&mut self, asksize: i64) {
    self.asksize = asksize;
  }

  pub fn with_asksize(mut self, asksize: i64) -> LastQuote {
    self.asksize = asksize;
    self
  }

  pub fn asksize(&self) -> &i64 {
    &self.asksize
  }


  pub fn set_askexchange(&mut self, askexchange: i64) {
    self.askexchange = askexchange;
  }

  pub fn with_askexchange(mut self, askexchange: i64) -> LastQuote {
    self.askexchange = askexchange;
    self
  }

  pub fn askexchange(&self) -> &i64 {
    &self.askexchange
  }


  pub fn set_bidprice(&mut self, bidprice: i64) {
    self.bidprice = bidprice;
  }

  pub fn with_bidprice(mut self, bidprice: i64) -> LastQuote {
    self.bidprice = bidprice;
    self
  }

  pub fn bidprice(&self) -> &i64 {
    &self.bidprice
  }


  pub fn set_bidsize(&mut self, bidsize: i64) {
    self.bidsize = bidsize;
  }

  pub fn with_bidsize(mut self, bidsize: i64) -> LastQuote {
    self.bidsize = bidsize;
    self
  }

  pub fn bidsize(&self) -> &i64 {
    &self.bidsize
  }


  pub fn set_bidexchange(&mut self, bidexchange: i64) {
    self.bidexchange = bidexchange;
  }

  pub fn with_bidexchange(mut self, bidexchange: i64) -> LastQuote {
    self.bidexchange = bidexchange;
    self
  }

  pub fn bidexchange(&self) -> &i64 {
    &self.bidexchange
  }


  pub fn set_timestamp(&mut self, timestamp: i64) {
    self.timestamp = timestamp;
  }

  pub fn with_timestamp(mut self, timestamp: i64) -> LastQuote {
    self.timestamp = timestamp;
    self
  }

  pub fn timestamp(&self) -> &i64 {
    &self.timestamp
  }


}


