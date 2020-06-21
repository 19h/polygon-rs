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
pub struct InlineResponse20020 {
  #[serde(rename = "status")]
  status: String,  // OK 
  #[serde(rename = "data")]
  data: Option<CryptoSnapshotTickerBook> 
}

impl InlineResponse20020 {
  pub fn new(status: String, ) -> InlineResponse20020 {
    InlineResponse20020 {
      status: status,
      data: None
    }
  }

  pub fn set_status(&mut self, status: String) {
    self.status = status;
  }

  pub fn with_status(mut self, status: String) -> InlineResponse20020 {
    self.status = status;
    self
  }

  pub fn status(&self) -> &String {
    &self.status
  }


  pub fn set_data(&mut self, data: CryptoSnapshotTickerBook) {
    self.data = Some(data);
  }

  pub fn with_data(mut self, data: CryptoSnapshotTickerBook) -> InlineResponse20020 {
    self.data = Some(data);
    self
  }

  pub fn data(&self) -> Option<&CryptoSnapshotTickerBook> {
    self.data.as_ref()
  }

  pub fn reset_data(&mut self) {
    self.data = None;
  }

}


