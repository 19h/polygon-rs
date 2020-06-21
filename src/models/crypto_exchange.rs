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
pub struct CryptoExchange {
  #[serde(rename = "id")]
  id: f32,  // 2.0 
  #[serde(rename = "type")]
  _type: String,  // exchange 
  #[serde(rename = "market")]
  market: String,  // crypto 
  #[serde(rename = "name")]
  name: String,  // GDAX 
  #[serde(rename = "url")]
  url: String  // https://www.gdax.com/ 
}

impl CryptoExchange {
  pub fn new(id: f32, _type: String, market: String, name: String, url: String, ) -> CryptoExchange {
    CryptoExchange {
      id: id,
      _type: _type,
      market: market,
      name: name,
      url: url
    }
  }

  pub fn set_id(&mut self, id: f32) {
    self.id = id;
  }

  pub fn with_id(mut self, id: f32) -> CryptoExchange {
    self.id = id;
    self
  }

  pub fn id(&self) -> &f32 {
    &self.id
  }


  pub fn set__type(&mut self, _type: String) {
    self._type = _type;
  }

  pub fn with__type(mut self, _type: String) -> CryptoExchange {
    self._type = _type;
    self
  }

  pub fn _type(&self) -> &String {
    &self._type
  }


  pub fn set_market(&mut self, market: String) {
    self.market = market;
  }

  pub fn with_market(mut self, market: String) -> CryptoExchange {
    self.market = market;
    self
  }

  pub fn market(&self) -> &String {
    &self.market
  }


  pub fn set_name(&mut self, name: String) {
    self.name = name;
  }

  pub fn with_name(mut self, name: String) -> CryptoExchange {
    self.name = name;
    self
  }

  pub fn name(&self) -> &String {
    &self.name
  }


  pub fn set_url(&mut self, url: String) {
    self.url = url;
  }

  pub fn with_url(mut self, url: String) -> CryptoExchange {
    self.url = url;
    self
  }

  pub fn url(&self) -> &String {
    &self.url
  }


}


