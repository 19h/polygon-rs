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
pub struct ForexSnapshotTicker {
  #[serde(rename = "ticker")]
  ticker: String,  // C:USDEUR 
  #[serde(rename = "day")]
  day: ForexSnapshotAgg, 
  #[serde(rename = "lastTrade")]
  last_trade: Forex, 
  #[serde(rename = "min")]
  min: ForexSnapshotAgg, 
  #[serde(rename = "prevDay")]
  prev_day: ForexSnapshotAgg, 
  #[serde(rename = "todaysChange")]
  todays_change: i64, 
  #[serde(rename = "todaysChangePerc")]
  todays_change_perc: i64, 
  #[serde(rename = "updated")]
  updated: i64  // 1547787608999 
}

impl ForexSnapshotTicker {
  pub fn new(ticker: String, day: ForexSnapshotAgg, last_trade: Forex, min: ForexSnapshotAgg, prev_day: ForexSnapshotAgg, todays_change: i64, todays_change_perc: i64, updated: i64, ) -> ForexSnapshotTicker {
    ForexSnapshotTicker {
      ticker: ticker,
      day: day,
      last_trade: last_trade,
      min: min,
      prev_day: prev_day,
      todays_change: todays_change,
      todays_change_perc: todays_change_perc,
      updated: updated
    }
  }

  pub fn set_ticker(&mut self, ticker: String) {
    self.ticker = ticker;
  }

  pub fn with_ticker(mut self, ticker: String) -> ForexSnapshotTicker {
    self.ticker = ticker;
    self
  }

  pub fn ticker(&self) -> &String {
    &self.ticker
  }


  pub fn set_day(&mut self, day: ForexSnapshotAgg) {
    self.day = day;
  }

  pub fn with_day(mut self, day: ForexSnapshotAgg) -> ForexSnapshotTicker {
    self.day = day;
    self
  }

  pub fn day(&self) -> &ForexSnapshotAgg {
    &self.day
  }


  pub fn set_last_trade(&mut self, last_trade: Forex) {
    self.last_trade = last_trade;
  }

  pub fn with_last_trade(mut self, last_trade: Forex) -> ForexSnapshotTicker {
    self.last_trade = last_trade;
    self
  }

  pub fn last_trade(&self) -> &Forex {
    &self.last_trade
  }


  pub fn set_min(&mut self, min: ForexSnapshotAgg) {
    self.min = min;
  }

  pub fn with_min(mut self, min: ForexSnapshotAgg) -> ForexSnapshotTicker {
    self.min = min;
    self
  }

  pub fn min(&self) -> &ForexSnapshotAgg {
    &self.min
  }


  pub fn set_prev_day(&mut self, prev_day: ForexSnapshotAgg) {
    self.prev_day = prev_day;
  }

  pub fn with_prev_day(mut self, prev_day: ForexSnapshotAgg) -> ForexSnapshotTicker {
    self.prev_day = prev_day;
    self
  }

  pub fn prev_day(&self) -> &ForexSnapshotAgg {
    &self.prev_day
  }


  pub fn set_todays_change(&mut self, todays_change: i64) {
    self.todays_change = todays_change;
  }

  pub fn with_todays_change(mut self, todays_change: i64) -> ForexSnapshotTicker {
    self.todays_change = todays_change;
    self
  }

  pub fn todays_change(&self) -> &i64 {
    &self.todays_change
  }


  pub fn set_todays_change_perc(&mut self, todays_change_perc: i64) {
    self.todays_change_perc = todays_change_perc;
  }

  pub fn with_todays_change_perc(mut self, todays_change_perc: i64) -> ForexSnapshotTicker {
    self.todays_change_perc = todays_change_perc;
    self
  }

  pub fn todays_change_perc(&self) -> &i64 {
    &self.todays_change_perc
  }


  pub fn set_updated(&mut self, updated: i64) {
    self.updated = updated;
  }

  pub fn with_updated(mut self, updated: i64) -> ForexSnapshotTicker {
    self.updated = updated;
    self
  }

  pub fn updated(&self) -> &i64 {
    &self.updated
  }


}


