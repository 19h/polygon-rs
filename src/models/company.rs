/* 
 * Polygon API
 *
 * The future of fintech.
 *
 * OpenAPI spec version: 1.0.1
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */
#![allow(unused_imports)]

use serde_json::Value;
use bigdecimal::BigDecimal;
use chrono::{NaiveDateTime, DateTime, FixedOffset, Utc};

use crate::models::*;
//use crate::date_serializer;
//use crate::datetime_serializer;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Company {
  #[serde(rename = "logo")]
  logo: Option<String>,  // https://s3.polygon.io/logos/aapl/logo.png 
  #[serde(rename = "exchange")]
  exchange: String,  // Nasdaq Global Select 
  #[serde(rename = "name")]
  name: String,  // Apple Inc. 
  #[serde(rename = "symbol")]
  symbol: String,  // AAPL 
  #[serde(rename = "listdate")]
  listdate: Option<String>, 
  #[serde(rename = "cik")]
  cik: Option<String>,  // 0000320193 
  #[serde(rename = "bloomberg")]
  bloomberg: Option<String>,  // EQ0010169500001000 
  #[serde(rename = "figi")]
  figi: Option<String>, 
  #[serde(rename = "lei")]
  lei: Option<String>,  // HWUPKR0MPOU8FGXBT394 
  #[serde(rename = "sic")]
  sic: Option<i64>,  // 3571 
  #[serde(rename = "country")]
  country: Option<String>,  // us 
  #[serde(rename = "industry")]
  industry: Option<String>,  // Computer Hardware 
  #[serde(rename = "sector")]
  sector: Option<String>,  // Technology 
  #[serde(rename = "marketcap")]
  marketcap: Option<i64>,  // 815604985500 
  #[serde(rename = "employees")]
  employees: Option<i64>,  // 116000 
  #[serde(rename = "phone")]
  phone: Option<String>,  // (408) 996-1010 
  #[serde(rename = "ceo")]
  ceo: Option<String>,  // Tim Cook 
  #[serde(rename = "url")]
  url: Option<String>,  // http://www.apple.com 
  #[serde(rename = "description")]
  description: String,  // Apple Inc. designs, manufactures, and markets mobile communication and media devices, personal computers, and portable digital music players to consumers...
 
  #[serde(rename = "similar")]
  similar: Option<Vec<String>>,  // ["MSFT","IBM","GOOGL"] 
  #[serde(rename = "tags")]
  tags: Option<Vec<String>>,  // ["Technology","Consumer Electronics","Computer Hardware"] 
  #[serde(rename = "updated")]
  updated: String 
}

impl Company {
  pub fn new(exchange: String, name: String, symbol: String, description: String, updated: String, ) -> Company {
    Company {
      logo: None,
      exchange: exchange,
      name: name,
      symbol: symbol,
      listdate: None,
      cik: None,
      bloomberg: None,
      figi: None,
      lei: None,
      sic: None,
      country: None,
      industry: None,
      sector: None,
      marketcap: None,
      employees: None,
      phone: None,
      ceo: None,
      url: None,
      description: description,
      similar: None,
      tags: None,
      updated: updated
    }
  }

  pub fn set_logo(&mut self, logo: String) {
    self.logo = Some(logo);
  }

  pub fn with_logo(mut self, logo: String) -> Company {
    self.logo = Some(logo);
    self
  }

  pub fn logo(&self) -> Option<&String> {
    self.logo.as_ref()
  }

  pub fn reset_logo(&mut self) {
    self.logo = None;
  }

  pub fn set_exchange(&mut self, exchange: String) {
    self.exchange = exchange;
  }

  pub fn with_exchange(mut self, exchange: String) -> Company {
    self.exchange = exchange;
    self
  }

  pub fn exchange(&self) -> &String {
    &self.exchange
  }


  pub fn set_name(&mut self, name: String) {
    self.name = name;
  }

  pub fn with_name(mut self, name: String) -> Company {
    self.name = name;
    self
  }

  pub fn name(&self) -> &String {
    &self.name
  }


  pub fn set_symbol(&mut self, symbol: String) {
    self.symbol = symbol;
  }

  pub fn with_symbol(mut self, symbol: String) -> Company {
    self.symbol = symbol;
    self
  }

  pub fn symbol(&self) -> &String {
    &self.symbol
  }


  pub fn set_listdate(&mut self, listdate: String) {
    self.listdate = Some(listdate);
  }

  pub fn with_listdate(mut self, listdate: String) -> Company {
    self.listdate = Some(listdate);
    self
  }

  pub fn listdate(&self) -> Option<&String> {
    self.listdate.as_ref()
  }

  pub fn reset_listdate(&mut self) {
    self.listdate = None;
  }

  pub fn set_cik(&mut self, cik: String) {
    self.cik = Some(cik);
  }

  pub fn with_cik(mut self, cik: String) -> Company {
    self.cik = Some(cik);
    self
  }

  pub fn cik(&self) -> Option<&String> {
    self.cik.as_ref()
  }

  pub fn reset_cik(&mut self) {
    self.cik = None;
  }

  pub fn set_bloomberg(&mut self, bloomberg: String) {
    self.bloomberg = Some(bloomberg);
  }

  pub fn with_bloomberg(mut self, bloomberg: String) -> Company {
    self.bloomberg = Some(bloomberg);
    self
  }

  pub fn bloomberg(&self) -> Option<&String> {
    self.bloomberg.as_ref()
  }

  pub fn reset_bloomberg(&mut self) {
    self.bloomberg = None;
  }

  pub fn set_figi(&mut self, figi: String) {
    self.figi = Some(figi);
  }

  pub fn with_figi(mut self, figi: String) -> Company {
    self.figi = Some(figi);
    self
  }

  pub fn figi(&self) -> Option<&String> {
    self.figi.as_ref()
  }

  pub fn reset_figi(&mut self) {
    self.figi = None;
  }

  pub fn set_lei(&mut self, lei: String) {
    self.lei = Some(lei);
  }

  pub fn with_lei(mut self, lei: String) -> Company {
    self.lei = Some(lei);
    self
  }

  pub fn lei(&self) -> Option<&String> {
    self.lei.as_ref()
  }

  pub fn reset_lei(&mut self) {
    self.lei = None;
  }

  pub fn set_sic(&mut self, sic: i64) {
    self.sic = Some(sic);
  }

  pub fn with_sic(mut self, sic: i64) -> Company {
    self.sic = Some(sic);
    self
  }

  pub fn sic(&self) -> Option<&i64> {
    self.sic.as_ref()
  }

  pub fn reset_sic(&mut self) {
    self.sic = None;
  }

  pub fn set_country(&mut self, country: String) {
    self.country = Some(country);
  }

  pub fn with_country(mut self, country: String) -> Company {
    self.country = Some(country);
    self
  }

  pub fn country(&self) -> Option<&String> {
    self.country.as_ref()
  }

  pub fn reset_country(&mut self) {
    self.country = None;
  }

  pub fn set_industry(&mut self, industry: String) {
    self.industry = Some(industry);
  }

  pub fn with_industry(mut self, industry: String) -> Company {
    self.industry = Some(industry);
    self
  }

  pub fn industry(&self) -> Option<&String> {
    self.industry.as_ref()
  }

  pub fn reset_industry(&mut self) {
    self.industry = None;
  }

  pub fn set_sector(&mut self, sector: String) {
    self.sector = Some(sector);
  }

  pub fn with_sector(mut self, sector: String) -> Company {
    self.sector = Some(sector);
    self
  }

  pub fn sector(&self) -> Option<&String> {
    self.sector.as_ref()
  }

  pub fn reset_sector(&mut self) {
    self.sector = None;
  }

  pub fn set_marketcap(&mut self, marketcap: i64) {
    self.marketcap = Some(marketcap);
  }

  pub fn with_marketcap(mut self, marketcap: i64) -> Company {
    self.marketcap = Some(marketcap);
    self
  }

  pub fn marketcap(&self) -> Option<&i64> {
    self.marketcap.as_ref()
  }

  pub fn reset_marketcap(&mut self) {
    self.marketcap = None;
  }

  pub fn set_employees(&mut self, employees: i64) {
    self.employees = Some(employees);
  }

  pub fn with_employees(mut self, employees: i64) -> Company {
    self.employees = Some(employees);
    self
  }

  pub fn employees(&self) -> Option<&i64> {
    self.employees.as_ref()
  }

  pub fn reset_employees(&mut self) {
    self.employees = None;
  }

  pub fn set_phone(&mut self, phone: String) {
    self.phone = Some(phone);
  }

  pub fn with_phone(mut self, phone: String) -> Company {
    self.phone = Some(phone);
    self
  }

  pub fn phone(&self) -> Option<&String> {
    self.phone.as_ref()
  }

  pub fn reset_phone(&mut self) {
    self.phone = None;
  }

  pub fn set_ceo(&mut self, ceo: String) {
    self.ceo = Some(ceo);
  }

  pub fn with_ceo(mut self, ceo: String) -> Company {
    self.ceo = Some(ceo);
    self
  }

  pub fn ceo(&self) -> Option<&String> {
    self.ceo.as_ref()
  }

  pub fn reset_ceo(&mut self) {
    self.ceo = None;
  }

  pub fn set_url(&mut self, url: String) {
    self.url = Some(url);
  }

  pub fn with_url(mut self, url: String) -> Company {
    self.url = Some(url);
    self
  }

  pub fn url(&self) -> Option<&String> {
    self.url.as_ref()
  }

  pub fn reset_url(&mut self) {
    self.url = None;
  }

  pub fn set_description(&mut self, description: String) {
    self.description = description;
  }

  pub fn with_description(mut self, description: String) -> Company {
    self.description = description;
    self
  }

  pub fn description(&self) -> &String {
    &self.description
  }


  pub fn set_similar(&mut self, similar: Vec<String>) {
    self.similar = Some(similar);
  }

  pub fn with_similar(mut self, similar: Vec<String>) -> Company {
    self.similar = Some(similar);
    self
  }

  pub fn similar(&self) -> Option<&Vec<String>> {
    self.similar.as_ref()
  }

  pub fn reset_similar(&mut self) {
    self.similar = None;
  }

  pub fn set_tags(&mut self, tags: Vec<String>) {
    self.tags = Some(tags);
  }

  pub fn with_tags(mut self, tags: Vec<String>) -> Company {
    self.tags = Some(tags);
    self
  }

  pub fn tags(&self) -> Option<&Vec<String>> {
    self.tags.as_ref()
  }

  pub fn reset_tags(&mut self) {
    self.tags = None;
  }

  pub fn set_updated(&mut self, updated: String) {
    self.updated = updated;
  }

  pub fn with_updated(mut self, updated: String) -> Company {
    self.updated = updated;
    self
  }

  pub fn updated(&self) -> &String {
    &self.updated
  }


}


