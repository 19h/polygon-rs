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
pub struct Exchange {
  #[serde(rename = "id")]
  id: f32,  // 2.0 
  #[serde(rename = "type")]
  _type: String,  // exchange 
  #[serde(rename = "market")]
  market: String,  // equities 
  #[serde(rename = "mic")]
  mic: String,  // XASE 
  #[serde(rename = "name")]
  name: String,  // NYSE American (AMEX) 
  #[serde(rename = "tape")]
  tape: String  // A 
}

impl Exchange {
  pub fn new(id: f32, _type: String, market: String, mic: String, name: String, tape: String, ) -> Exchange {
    Exchange {
      id: id,
      _type: _type,
      market: market,
      mic: mic,
      name: name,
      tape: tape
    }
  }

  pub fn set_id(&mut self, id: f32) {
    self.id = id;
  }

  pub fn with_id(mut self, id: f32) -> Exchange {
    self.id = id;
    self
  }

  pub fn id(&self) -> &f32 {
    &self.id
  }


  pub fn set__type(&mut self, _type: String) {
    self._type = _type;
  }

  pub fn with__type(mut self, _type: String) -> Exchange {
    self._type = _type;
    self
  }

  pub fn _type(&self) -> &String {
    &self._type
  }


  pub fn set_market(&mut self, market: String) {
    self.market = market;
  }

  pub fn with_market(mut self, market: String) -> Exchange {
    self.market = market;
    self
  }

  pub fn market(&self) -> &String {
    &self.market
  }


  pub fn set_mic(&mut self, mic: String) {
    self.mic = mic;
  }

  pub fn with_mic(mut self, mic: String) -> Exchange {
    self.mic = mic;
    self
  }

  pub fn mic(&self) -> &String {
    &self.mic
  }


  pub fn set_name(&mut self, name: String) {
    self.name = name;
  }

  pub fn with_name(mut self, name: String) -> Exchange {
    self.name = name;
    self
  }

  pub fn name(&self) -> &String {
    &self.name
  }


  pub fn set_tape(&mut self, tape: String) {
    self.tape = tape;
  }

  pub fn with_tape(mut self, tape: String) -> Exchange {
    self.tape = tape;
    self
  }

  pub fn tape(&self) -> &String {
    &self.tape
  }


}


