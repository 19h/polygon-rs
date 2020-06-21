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
pub struct RatingSection {
  #[serde(rename = "current")]
  current: f32,  // 0.0 
  #[serde(rename = "month1")]
  month1: f32,  // 1.0 
  #[serde(rename = "month2")]
  month2: f32,  // 3.0 
  #[serde(rename = "month3")]
  month3: f32,  // 4.0 
  #[serde(rename = "month4")]
  month4: Option<f32>,  // 3.0 
  #[serde(rename = "month5")]
  month5: Option<f32>  // 2.0 
}

impl RatingSection {
  pub fn new(current: f32, month1: f32, month2: f32, month3: f32, ) -> RatingSection {
    RatingSection {
      current: current,
      month1: month1,
      month2: month2,
      month3: month3,
      month4: None,
      month5: None
    }
  }

  pub fn set_current(&mut self, current: f32) {
    self.current = current;
  }

  pub fn with_current(mut self, current: f32) -> RatingSection {
    self.current = current;
    self
  }

  pub fn current(&self) -> &f32 {
    &self.current
  }


  pub fn set_month1(&mut self, month1: f32) {
    self.month1 = month1;
  }

  pub fn with_month1(mut self, month1: f32) -> RatingSection {
    self.month1 = month1;
    self
  }

  pub fn month1(&self) -> &f32 {
    &self.month1
  }


  pub fn set_month2(&mut self, month2: f32) {
    self.month2 = month2;
  }

  pub fn with_month2(mut self, month2: f32) -> RatingSection {
    self.month2 = month2;
    self
  }

  pub fn month2(&self) -> &f32 {
    &self.month2
  }


  pub fn set_month3(&mut self, month3: f32) {
    self.month3 = month3;
  }

  pub fn with_month3(mut self, month3: f32) -> RatingSection {
    self.month3 = month3;
    self
  }

  pub fn month3(&self) -> &f32 {
    &self.month3
  }


  pub fn set_month4(&mut self, month4: f32) {
    self.month4 = Some(month4);
  }

  pub fn with_month4(mut self, month4: f32) -> RatingSection {
    self.month4 = Some(month4);
    self
  }

  pub fn month4(&self) -> Option<&f32> {
    self.month4.as_ref()
  }

  pub fn reset_month4(&mut self) {
    self.month4 = None;
  }

  pub fn set_month5(&mut self, month5: f32) {
    self.month5 = Some(month5);
  }

  pub fn with_month5(mut self, month5: f32) -> RatingSection {
    self.month5 = Some(month5);
    self
  }

  pub fn month5(&self) -> Option<&f32> {
    self.month5.as_ref()
  }

  pub fn reset_month5(&mut self) {
    self.month5 = None;
  }

}


