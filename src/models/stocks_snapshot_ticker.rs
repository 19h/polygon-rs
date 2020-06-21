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
pub struct StocksSnapshotTicker {
  #[serde(rename = "ticker")]
  ticker: String,  // AAPL 
  #[serde(rename = "day")]
  day: StocksSnapshotAgg, 
  #[serde(rename = "lastTrade")]
  last_trade: Trade, 
  #[serde(rename = "lastQuote")]
  last_quote: Option<StocksSnapshotQuote>, 
  #[serde(rename = "min")]
  min: StocksSnapshotAgg, 
  #[serde(rename = "prevDay")]
  prev_day: StocksSnapshotAgg, 
  #[serde(rename = "todaysChange")]
  todays_change: i64, 
  #[serde(rename = "todaysChangePerc")]
  todays_change_perc: i64, 
  #[serde(rename = "updated")]
  updated: i64  // 1547787608999 
}

impl StocksSnapshotTicker {
  pub fn new(ticker: String, day: StocksSnapshotAgg, last_trade: Trade, min: StocksSnapshotAgg, prev_day: StocksSnapshotAgg, todays_change: i64, todays_change_perc: i64, updated: i64, ) -> StocksSnapshotTicker {
    StocksSnapshotTicker {
      ticker: ticker,
      day: day,
      last_trade: last_trade,
      last_quote: None,
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

  pub fn with_ticker(mut self, ticker: String) -> StocksSnapshotTicker {
    self.ticker = ticker;
    self
  }

  pub fn ticker(&self) -> &String {
    &self.ticker
  }


  pub fn set_day(&mut self, day: StocksSnapshotAgg) {
    self.day = day;
  }

  pub fn with_day(mut self, day: StocksSnapshotAgg) -> StocksSnapshotTicker {
    self.day = day;
    self
  }

  pub fn day(&self) -> &StocksSnapshotAgg {
    &self.day
  }


  pub fn set_last_trade(&mut self, last_trade: Trade) {
    self.last_trade = last_trade;
  }

  pub fn with_last_trade(mut self, last_trade: Trade) -> StocksSnapshotTicker {
    self.last_trade = last_trade;
    self
  }

  pub fn last_trade(&self) -> &Trade {
    &self.last_trade
  }


  pub fn set_last_quote(&mut self, last_quote: StocksSnapshotQuote) {
    self.last_quote = Some(last_quote);
  }

  pub fn with_last_quote(mut self, last_quote: StocksSnapshotQuote) -> StocksSnapshotTicker {
    self.last_quote = Some(last_quote);
    self
  }

  pub fn last_quote(&self) -> Option<&StocksSnapshotQuote> {
    self.last_quote.as_ref()
  }

  pub fn reset_last_quote(&mut self) {
    self.last_quote = None;
  }

  pub fn set_min(&mut self, min: StocksSnapshotAgg) {
    self.min = min;
  }

  pub fn with_min(mut self, min: StocksSnapshotAgg) -> StocksSnapshotTicker {
    self.min = min;
    self
  }

  pub fn min(&self) -> &StocksSnapshotAgg {
    &self.min
  }


  pub fn set_prev_day(&mut self, prev_day: StocksSnapshotAgg) {
    self.prev_day = prev_day;
  }

  pub fn with_prev_day(mut self, prev_day: StocksSnapshotAgg) -> StocksSnapshotTicker {
    self.prev_day = prev_day;
    self
  }

  pub fn prev_day(&self) -> &StocksSnapshotAgg {
    &self.prev_day
  }


  pub fn set_todays_change(&mut self, todays_change: i64) {
    self.todays_change = todays_change;
  }

  pub fn with_todays_change(mut self, todays_change: i64) -> StocksSnapshotTicker {
    self.todays_change = todays_change;
    self
  }

  pub fn todays_change(&self) -> &i64 {
    &self.todays_change
  }


  pub fn set_todays_change_perc(&mut self, todays_change_perc: i64) {
    self.todays_change_perc = todays_change_perc;
  }

  pub fn with_todays_change_perc(mut self, todays_change_perc: i64) -> StocksSnapshotTicker {
    self.todays_change_perc = todays_change_perc;
    self
  }

  pub fn todays_change_perc(&self) -> &i64 {
    &self.todays_change_perc
  }


  pub fn set_updated(&mut self, updated: i64) {
    self.updated = updated;
  }

  pub fn with_updated(mut self, updated: i64) -> StocksSnapshotTicker {
    self.updated = updated;
    self
  }

  pub fn updated(&self) -> &i64 {
    &self.updated
  }


}


