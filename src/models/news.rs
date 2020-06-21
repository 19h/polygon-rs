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
pub struct News {
  #[serde(rename = "symbols")]
  symbols: Vec<StockSymbol>,  // ["MSFT","AAPL","IBM"] 
  #[serde(rename = "title")]
  title: String,  // Goldman in talks to finance iPhones - WSJ 
  #[serde(rename = "url")]
  url: String,  // https://seekingalpha.com/news/3328826-goldman-talks-finance-iphones-wsj 
  #[serde(rename = "source")]
  source: String,  // SeekingAlpha 
  #[serde(rename = "summary")]
  summary: String,  // Continuing its push into more traditional areas of consumer finance, Goldman Sachs (NYSE:GS) is reportedly in talks with Apple (NASDAQ:AAPL) to finance iPhone purchases.Buyers theoretically would be a... 
  #[serde(rename = "image")]
  image: Option<String>,  // https://static.seekingalpha.com/assets/og_image_410-b8960ce31ec84f7f12dba11a09fc1849b69b234e0f5f39d7c62f46f8692e58a5.png 
  #[serde(rename = "timestamp")]
  timestamp: DateTime<Utc>,  // 2018-02-07T12:48:47Z 
  #[serde(rename = "keywords")]
  keywords: Option<Vec<Value>>  // ["financial services","aapl","investing","bsiness news","mobile"] 
}

impl News {
  pub fn new(symbols: Vec<StockSymbol>, title: String, url: String, source: String, summary: String, timestamp: DateTime<Utc>, ) -> News {
    News {
      symbols: symbols,
      title: title,
      url: url,
      source: source,
      summary: summary,
      image: None,
      timestamp: timestamp,
      keywords: None
    }
  }

  pub fn set_symbols(&mut self, symbols: Vec<StockSymbol>) {
    self.symbols = symbols;
  }

  pub fn with_symbols(mut self, symbols: Vec<StockSymbol>) -> News {
    self.symbols = symbols;
    self
  }

  pub fn symbols(&self) -> &Vec<StockSymbol> {
    &self.symbols
  }


  pub fn set_title(&mut self, title: String) {
    self.title = title;
  }

  pub fn with_title(mut self, title: String) -> News {
    self.title = title;
    self
  }

  pub fn title(&self) -> &String {
    &self.title
  }


  pub fn set_url(&mut self, url: String) {
    self.url = url;
  }

  pub fn with_url(mut self, url: String) -> News {
    self.url = url;
    self
  }

  pub fn url(&self) -> &String {
    &self.url
  }


  pub fn set_source(&mut self, source: String) {
    self.source = source;
  }

  pub fn with_source(mut self, source: String) -> News {
    self.source = source;
    self
  }

  pub fn source(&self) -> &String {
    &self.source
  }


  pub fn set_summary(&mut self, summary: String) {
    self.summary = summary;
  }

  pub fn with_summary(mut self, summary: String) -> News {
    self.summary = summary;
    self
  }

  pub fn summary(&self) -> &String {
    &self.summary
  }


  pub fn set_image(&mut self, image: String) {
    self.image = Some(image);
  }

  pub fn with_image(mut self, image: String) -> News {
    self.image = Some(image);
    self
  }

  pub fn image(&self) -> Option<&String> {
    self.image.as_ref()
  }

  pub fn reset_image(&mut self) {
    self.image = None;
  }

  pub fn set_timestamp(&mut self, timestamp: DateTime<Utc>) {
    self.timestamp = timestamp;
  }

  pub fn with_timestamp(mut self, timestamp: DateTime<Utc>) -> News {
    self.timestamp = timestamp;
    self
  }

  pub fn timestamp(&self) -> &DateTime<Utc> {
    &self.timestamp
  }


  pub fn set_keywords(&mut self, keywords: Vec<Value>) {
    self.keywords = Some(keywords);
  }

  pub fn with_keywords(mut self, keywords: Vec<Value>) -> News {
    self.keywords = Some(keywords);
    self
  }

  pub fn keywords(&self) -> Option<&Vec<Value>> {
    self.keywords.as_ref()
  }

  pub fn reset_keywords(&mut self) {
    self.keywords = None;
  }

}


