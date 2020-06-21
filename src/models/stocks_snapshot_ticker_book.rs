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
pub struct StocksSnapshotTickerBook {
  #[serde(rename = "ticker")]
  ticker: String,  // AAPL 
  #[serde(rename = "bids")]
  bids: Option<Vec<StocksSnapshotBookItem>>, 
  #[serde(rename = "asks")]
  asks: Option<Vec<StocksSnapshotBookItem>>, 
  #[serde(rename = "bidCount")]
  bid_count: Option<i64>, 
  #[serde(rename = "askCount")]
  ask_count: Option<i64>, 
  #[serde(rename = "spread")]
  spread: Option<i64>, 
  #[serde(rename = "updated")]
  updated: i64  // 1547787608999 
}

impl StocksSnapshotTickerBook {
  pub fn new(ticker: String, updated: i64, ) -> StocksSnapshotTickerBook {
    StocksSnapshotTickerBook {
      ticker: ticker,
      bids: None,
      asks: None,
      bid_count: None,
      ask_count: None,
      spread: None,
      updated: updated
    }
  }

  pub fn set_ticker(&mut self, ticker: String) {
    self.ticker = ticker;
  }

  pub fn with_ticker(mut self, ticker: String) -> StocksSnapshotTickerBook {
    self.ticker = ticker;
    self
  }

  pub fn ticker(&self) -> &String {
    &self.ticker
  }


  pub fn set_bids(&mut self, bids: Vec<StocksSnapshotBookItem>) {
    self.bids = Some(bids);
  }

  pub fn with_bids(mut self, bids: Vec<StocksSnapshotBookItem>) -> StocksSnapshotTickerBook {
    self.bids = Some(bids);
    self
  }

  pub fn bids(&self) -> Option<&Vec<StocksSnapshotBookItem>> {
    self.bids.as_ref()
  }

  pub fn reset_bids(&mut self) {
    self.bids = None;
  }

  pub fn set_asks(&mut self, asks: Vec<StocksSnapshotBookItem>) {
    self.asks = Some(asks);
  }

  pub fn with_asks(mut self, asks: Vec<StocksSnapshotBookItem>) -> StocksSnapshotTickerBook {
    self.asks = Some(asks);
    self
  }

  pub fn asks(&self) -> Option<&Vec<StocksSnapshotBookItem>> {
    self.asks.as_ref()
  }

  pub fn reset_asks(&mut self) {
    self.asks = None;
  }

  pub fn set_bid_count(&mut self, bid_count: i64) {
    self.bid_count = Some(bid_count);
  }

  pub fn with_bid_count(mut self, bid_count: i64) -> StocksSnapshotTickerBook {
    self.bid_count = Some(bid_count);
    self
  }

  pub fn bid_count(&self) -> Option<&i64> {
    self.bid_count.as_ref()
  }

  pub fn reset_bid_count(&mut self) {
    self.bid_count = None;
  }

  pub fn set_ask_count(&mut self, ask_count: i64) {
    self.ask_count = Some(ask_count);
  }

  pub fn with_ask_count(mut self, ask_count: i64) -> StocksSnapshotTickerBook {
    self.ask_count = Some(ask_count);
    self
  }

  pub fn ask_count(&self) -> Option<&i64> {
    self.ask_count.as_ref()
  }

  pub fn reset_ask_count(&mut self) {
    self.ask_count = None;
  }

  pub fn set_spread(&mut self, spread: i64) {
    self.spread = Some(spread);
  }

  pub fn with_spread(mut self, spread: i64) -> StocksSnapshotTickerBook {
    self.spread = Some(spread);
    self
  }

  pub fn spread(&self) -> Option<&i64> {
    self.spread.as_ref()
  }

  pub fn reset_spread(&mut self) {
    self.spread = None;
  }

  pub fn set_updated(&mut self, updated: i64) {
    self.updated = updated;
  }

  pub fn with_updated(mut self, updated: i64) -> StocksSnapshotTickerBook {
    self.updated = updated;
    self
  }

  pub fn updated(&self) -> &i64 {
    &self.updated
  }


}


