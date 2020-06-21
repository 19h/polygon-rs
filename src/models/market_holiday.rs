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
pub struct MarketHoliday {
  #[serde(rename = "exchange")]
  exchange: String,  // NYSE 
  #[serde(rename = "name")]
  name: String,  // Thanksgiving Day 
  #[serde(rename = "status")]
  status: String,  // early-close 
  #[serde(rename = "date")]
  date: DateTime<Utc>,  // 2018-11-23T00:00Z 
  #[serde(rename = "open")]
  open: Option<DateTime<Utc>>,  // 2018-11-23T09:30Z 
  #[serde(rename = "close")]
  close: Option<DateTime<Utc>>  // 2018-11-23T13:00Z 
}

impl MarketHoliday {
  pub fn new(exchange: String, name: String, status: String, date: DateTime<Utc>, ) -> MarketHoliday {
    MarketHoliday {
      exchange: exchange,
      name: name,
      status: status,
      date: date,
      open: None,
      close: None
    }
  }

  pub fn set_exchange(&mut self, exchange: String) {
    self.exchange = exchange;
  }

  pub fn with_exchange(mut self, exchange: String) -> MarketHoliday {
    self.exchange = exchange;
    self
  }

  pub fn exchange(&self) -> &String {
    &self.exchange
  }


  pub fn set_name(&mut self, name: String) {
    self.name = name;
  }

  pub fn with_name(mut self, name: String) -> MarketHoliday {
    self.name = name;
    self
  }

  pub fn name(&self) -> &String {
    &self.name
  }


  pub fn set_status(&mut self, status: String) {
    self.status = status;
  }

  pub fn with_status(mut self, status: String) -> MarketHoliday {
    self.status = status;
    self
  }

  pub fn status(&self) -> &String {
    &self.status
  }


  pub fn set_date(&mut self, date: DateTime<Utc>) {
    self.date = date;
  }

  pub fn with_date(mut self, date: DateTime<Utc>) -> MarketHoliday {
    self.date = date;
    self
  }

  pub fn date(&self) -> &DateTime<Utc> {
    &self.date
  }


  pub fn set_open(&mut self, open: DateTime<Utc>) {
    self.open = Some(open);
  }

  pub fn with_open(mut self, open: DateTime<Utc>) -> MarketHoliday {
    self.open = Some(open);
    self
  }

  pub fn open(&self) -> Option<&DateTime<Utc>> {
    self.open.as_ref()
  }

  pub fn reset_open(&mut self) {
    self.open = None;
  }

  pub fn set_close(&mut self, close: DateTime<Utc>) {
    self.close = Some(close);
  }

  pub fn with_close(mut self, close: DateTime<Utc>) -> MarketHoliday {
    self.close = Some(close);
    self
  }

  pub fn close(&self) -> Option<&DateTime<Utc>> {
    self.close.as_ref()
  }

  pub fn reset_close(&mut self) {
    self.close = None;
  }

}


