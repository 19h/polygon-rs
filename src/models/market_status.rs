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
pub struct MarketStatus {
  #[serde(rename = "market")]
  market: String,  // open 
  #[serde(rename = "serverTime")]
  server_time: DateTime<Utc>,  // 2018-07-19T08:51:07-04:00 
  #[serde(rename = "exchanges")]
  exchanges: MarketStatusExchanges, 
  #[serde(rename = "currencies")]
  currencies: Option<MarketStatusCurrencies> 
}

impl MarketStatus {
  pub fn new(market: String, server_time: DateTime<Utc>, exchanges: MarketStatusExchanges, ) -> MarketStatus {
    MarketStatus {
      market: market,
      server_time: server_time,
      exchanges: exchanges,
      currencies: None
    }
  }

  pub fn set_market(&mut self, market: String) {
    self.market = market;
  }

  pub fn with_market(mut self, market: String) -> MarketStatus {
    self.market = market;
    self
  }

  pub fn market(&self) -> &String {
    &self.market
  }


  pub fn set_server_time(&mut self, server_time: DateTime<Utc>) {
    self.server_time = server_time;
  }

  pub fn with_server_time(mut self, server_time: DateTime<Utc>) -> MarketStatus {
    self.server_time = server_time;
    self
  }

  pub fn server_time(&self) -> &DateTime<Utc> {
    &self.server_time
  }


  pub fn set_exchanges(&mut self, exchanges: MarketStatusExchanges) {
    self.exchanges = exchanges;
  }

  pub fn with_exchanges(mut self, exchanges: MarketStatusExchanges) -> MarketStatus {
    self.exchanges = exchanges;
    self
  }

  pub fn exchanges(&self) -> &MarketStatusExchanges {
    &self.exchanges
  }


  pub fn set_currencies(&mut self, currencies: MarketStatusCurrencies) {
    self.currencies = Some(currencies);
  }

  pub fn with_currencies(mut self, currencies: MarketStatusCurrencies) -> MarketStatus {
    self.currencies = Some(currencies);
    self
  }

  pub fn currencies(&self) -> Option<&MarketStatusCurrencies> {
    self.currencies.as_ref()
  }

  pub fn reset_currencies(&mut self) {
    self.currencies = None;
  }

}


