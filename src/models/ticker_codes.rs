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
/// TickerCodes : Additional details about this ticker. No schema.

use serde_json::Value;
use bigdecimal::BigDecimal;
use chrono::{NaiveDateTime, DateTime, FixedOffset, Utc};

use crate::models::*;
//use crate::date_serializer;
//use crate::datetime_serializer;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TickerCodes {
  #[serde(rename = "cik")]
  cik: Option<String>,  // 0000320193 
  #[serde(rename = "figi")]
  figi: Option<String>,  // BBG000B9Y5X2 
  #[serde(rename = "cfigi")]
  cfigi: Option<String>,  // BBG000B9XRY4 
  #[serde(rename = "scfigi")]
  scfigi: Option<String>,  // BBG001S5N8V8 
  #[serde(rename = "figiuid")]
  figiuid: Option<String>  // EQ0010169500001000 
}

impl TickerCodes {
  pub fn new() -> TickerCodes {
    TickerCodes {
      cik: None,
      figi: None,
      cfigi: None,
      scfigi: None,
      figiuid: None
    }
  }

  pub fn set_cik(&mut self, cik: String) {
    self.cik = Some(cik);
  }

  pub fn with_cik(mut self, cik: String) -> TickerCodes {
    self.cik = Some(cik);
    self
  }

  pub fn cik(&self) -> Option<&String> {
    self.cik.as_ref()
  }

  pub fn reset_cik(&mut self) {
    self.cik = None;
  }

  pub fn set_figi(&mut self, figi: String) {
    self.figi = Some(figi);
  }

  pub fn with_figi(mut self, figi: String) -> TickerCodes {
    self.figi = Some(figi);
    self
  }

  pub fn figi(&self) -> Option<&String> {
    self.figi.as_ref()
  }

  pub fn reset_figi(&mut self) {
    self.figi = None;
  }

  pub fn set_cfigi(&mut self, cfigi: String) {
    self.cfigi = Some(cfigi);
  }

  pub fn with_cfigi(mut self, cfigi: String) -> TickerCodes {
    self.cfigi = Some(cfigi);
    self
  }

  pub fn cfigi(&self) -> Option<&String> {
    self.cfigi.as_ref()
  }

  pub fn reset_cfigi(&mut self) {
    self.cfigi = None;
  }

  pub fn set_scfigi(&mut self, scfigi: String) {
    self.scfigi = Some(scfigi);
  }

  pub fn with_scfigi(mut self, scfigi: String) -> TickerCodes {
    self.scfigi = Some(scfigi);
    self
  }

  pub fn scfigi(&self) -> Option<&String> {
    self.scfigi.as_ref()
  }

  pub fn reset_scfigi(&mut self) {
    self.scfigi = None;
  }

  pub fn set_figiuid(&mut self, figiuid: String) {
    self.figiuid = Some(figiuid);
  }

  pub fn with_figiuid(mut self, figiuid: String) -> TickerCodes {
    self.figiuid = Some(figiuid);
    self
  }

  pub fn figiuid(&self) -> Option<&String> {
    self.figiuid.as_ref()
  }

  pub fn reset_figiuid(&mut self) {
    self.figiuid = None;
  }

}


