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
pub struct Ticker {
  #[serde(rename = "ticker")]
  ticker: StockSymbol, 
  #[serde(rename = "name")]
  name: String,  // Apple Inc 
  #[serde(rename = "market")]
  market: String,  // STOCKS 
  #[serde(rename = "locale")]
  locale: String,  // US 
  #[serde(rename = "currency")]
  currency: Option<String>,  // USD 
  #[serde(rename = "active")]
  active: Option<bool>,  // true 
  #[serde(rename = "primaryExch")]
  primary_exch: Option<String>,  // NGS 
  #[serde(rename = "url")]
  url: Option<String>,  // https://api.polygon.io/v2/reference/tickers/AAPL 
  #[serde(rename = "updated")]
  updated: DateTime<Utc>, 
  #[serde(rename = "attrs")]
  attrs: Option<Value>, 
  #[serde(rename = "codes")]
  codes: Option<TickerCodes> 
}

impl Ticker {
  pub fn new(ticker: StockSymbol, name: String, market: String, locale: String, updated: DateTime<Utc>, ) -> Ticker {
    Ticker {
      ticker: ticker,
      name: name,
      market: market,
      locale: locale,
      currency: None,
      active: None,
      primary_exch: None,
      url: None,
      updated: updated,
      attrs: None,
      codes: None
    }
  }

  pub fn set_ticker(&mut self, ticker: StockSymbol) {
    self.ticker = ticker;
  }

  pub fn with_ticker(mut self, ticker: StockSymbol) -> Ticker {
    self.ticker = ticker;
    self
  }

  pub fn ticker(&self) -> &StockSymbol {
    &self.ticker
  }


  pub fn set_name(&mut self, name: String) {
    self.name = name;
  }

  pub fn with_name(mut self, name: String) -> Ticker {
    self.name = name;
    self
  }

  pub fn name(&self) -> &String {
    &self.name
  }


  pub fn set_market(&mut self, market: String) {
    self.market = market;
  }

  pub fn with_market(mut self, market: String) -> Ticker {
    self.market = market;
    self
  }

  pub fn market(&self) -> &String {
    &self.market
  }


  pub fn set_locale(&mut self, locale: String) {
    self.locale = locale;
  }

  pub fn with_locale(mut self, locale: String) -> Ticker {
    self.locale = locale;
    self
  }

  pub fn locale(&self) -> &String {
    &self.locale
  }


  pub fn set_currency(&mut self, currency: String) {
    self.currency = Some(currency);
  }

  pub fn with_currency(mut self, currency: String) -> Ticker {
    self.currency = Some(currency);
    self
  }

  pub fn currency(&self) -> Option<&String> {
    self.currency.as_ref()
  }

  pub fn reset_currency(&mut self) {
    self.currency = None;
  }

  pub fn set_active(&mut self, active: bool) {
    self.active = Some(active);
  }

  pub fn with_active(mut self, active: bool) -> Ticker {
    self.active = Some(active);
    self
  }

  pub fn active(&self) -> Option<&bool> {
    self.active.as_ref()
  }

  pub fn reset_active(&mut self) {
    self.active = None;
  }

  pub fn set_primary_exch(&mut self, primary_exch: String) {
    self.primary_exch = Some(primary_exch);
  }

  pub fn with_primary_exch(mut self, primary_exch: String) -> Ticker {
    self.primary_exch = Some(primary_exch);
    self
  }

  pub fn primary_exch(&self) -> Option<&String> {
    self.primary_exch.as_ref()
  }

  pub fn reset_primary_exch(&mut self) {
    self.primary_exch = None;
  }

  pub fn set_url(&mut self, url: String) {
    self.url = Some(url);
  }

  pub fn with_url(mut self, url: String) -> Ticker {
    self.url = Some(url);
    self
  }

  pub fn url(&self) -> Option<&String> {
    self.url.as_ref()
  }

  pub fn reset_url(&mut self) {
    self.url = None;
  }

  pub fn set_updated(&mut self, updated: DateTime<Utc>) {
    self.updated = updated;
  }

  pub fn with_updated(mut self, updated: DateTime<Utc>) -> Ticker {
    self.updated = updated;
    self
  }

  pub fn updated(&self) -> &DateTime<Utc> {
    &self.updated
  }


  pub fn set_attrs(&mut self, attrs: Value) {
    self.attrs = Some(attrs);
  }

  pub fn with_attrs(mut self, attrs: Value) -> Ticker {
    self.attrs = Some(attrs);
    self
  }

  pub fn attrs(&self) -> Option<&Value> {
    self.attrs.as_ref()
  }

  pub fn reset_attrs(&mut self) {
    self.attrs = None;
  }

  pub fn set_codes(&mut self, codes: TickerCodes) {
    self.codes = Some(codes);
  }

  pub fn with_codes(mut self, codes: TickerCodes) -> Ticker {
    self.codes = Some(codes);
    self
  }

  pub fn codes(&self) -> Option<&TickerCodes> {
    self.codes.as_ref()
  }

  pub fn reset_codes(&mut self) {
    self.codes = None;
  }

}


