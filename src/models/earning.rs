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
pub struct Earning {
  #[serde(rename = "symbol")]
  symbol: String,  // AAPL 
  #[serde(rename = "EPSReportDate")]
  eps_report_date: DateTime<Utc>,  // 2018-02-01T00:00Z 
  #[serde(rename = "EPSReportDateStr")]
  eps_report_date_str: String,  // 2018-02-01 
  #[serde(rename = "fiscalPeriod")]
  fiscal_period: Option<String>,  // Q1 2018 
  #[serde(rename = "fiscalEndDate")]
  fiscal_end_date: Option<DateTime<Utc>>,  // 2017-12-31T00:00Z 
  #[serde(rename = "actualEPS")]
  actual_eps: Option<f32>,  // 3.89 
  #[serde(rename = "consensusEPS")]
  consensus_eps: Option<f32>,  // 3.82 
  #[serde(rename = "estimatedEPS")]
  estimated_eps: Option<f32>,  // 3.82 
  #[serde(rename = "announceTime")]
  announce_time: Option<String>,  // AMC 
  #[serde(rename = "numberOfEstimates")]
  number_of_estimates: Option<f32>,  // 9.0 
  #[serde(rename = "EPSSurpriseDollar")]
  eps_surprise_dollar: Option<f32>,  // 0.07 
  #[serde(rename = "yearAgo")]
  year_ago: Option<f32>,  // 3.36 
  #[serde(rename = "yearAgoChangePercent")]
  year_ago_change_percent: Option<f32>,  // 16.0 
  #[serde(rename = "estimatedChangePercent")]
  estimated_change_percent: Option<f32>  // 14.0 
}

impl Earning {
  pub fn new(symbol: String, eps_report_date: DateTime<Utc>, eps_report_date_str: String, ) -> Earning {
    Earning {
      symbol: symbol,
      eps_report_date: eps_report_date,
      eps_report_date_str: eps_report_date_str,
      fiscal_period: None,
      fiscal_end_date: None,
      actual_eps: None,
      consensus_eps: None,
      estimated_eps: None,
      announce_time: None,
      number_of_estimates: None,
      eps_surprise_dollar: None,
      year_ago: None,
      year_ago_change_percent: None,
      estimated_change_percent: None
    }
  }

  pub fn set_symbol(&mut self, symbol: String) {
    self.symbol = symbol;
  }

  pub fn with_symbol(mut self, symbol: String) -> Earning {
    self.symbol = symbol;
    self
  }

  pub fn symbol(&self) -> &String {
    &self.symbol
  }


  pub fn set_eps_report_date(&mut self, eps_report_date: DateTime<Utc>) {
    self.eps_report_date = eps_report_date;
  }

  pub fn with_eps_report_date(mut self, eps_report_date: DateTime<Utc>) -> Earning {
    self.eps_report_date = eps_report_date;
    self
  }

  pub fn eps_report_date(&self) -> &DateTime<Utc> {
    &self.eps_report_date
  }


  pub fn set_eps_report_date_str(&mut self, eps_report_date_str: String) {
    self.eps_report_date_str = eps_report_date_str;
  }

  pub fn with_eps_report_date_str(mut self, eps_report_date_str: String) -> Earning {
    self.eps_report_date_str = eps_report_date_str;
    self
  }

  pub fn eps_report_date_str(&self) -> &String {
    &self.eps_report_date_str
  }


  pub fn set_fiscal_period(&mut self, fiscal_period: String) {
    self.fiscal_period = Some(fiscal_period);
  }

  pub fn with_fiscal_period(mut self, fiscal_period: String) -> Earning {
    self.fiscal_period = Some(fiscal_period);
    self
  }

  pub fn fiscal_period(&self) -> Option<&String> {
    self.fiscal_period.as_ref()
  }

  pub fn reset_fiscal_period(&mut self) {
    self.fiscal_period = None;
  }

  pub fn set_fiscal_end_date(&mut self, fiscal_end_date: DateTime<Utc>) {
    self.fiscal_end_date = Some(fiscal_end_date);
  }

  pub fn with_fiscal_end_date(mut self, fiscal_end_date: DateTime<Utc>) -> Earning {
    self.fiscal_end_date = Some(fiscal_end_date);
    self
  }

  pub fn fiscal_end_date(&self) -> Option<&DateTime<Utc>> {
    self.fiscal_end_date.as_ref()
  }

  pub fn reset_fiscal_end_date(&mut self) {
    self.fiscal_end_date = None;
  }

  pub fn set_actual_eps(&mut self, actual_eps: f32) {
    self.actual_eps = Some(actual_eps);
  }

  pub fn with_actual_eps(mut self, actual_eps: f32) -> Earning {
    self.actual_eps = Some(actual_eps);
    self
  }

  pub fn actual_eps(&self) -> Option<&f32> {
    self.actual_eps.as_ref()
  }

  pub fn reset_actual_eps(&mut self) {
    self.actual_eps = None;
  }

  pub fn set_consensus_eps(&mut self, consensus_eps: f32) {
    self.consensus_eps = Some(consensus_eps);
  }

  pub fn with_consensus_eps(mut self, consensus_eps: f32) -> Earning {
    self.consensus_eps = Some(consensus_eps);
    self
  }

  pub fn consensus_eps(&self) -> Option<&f32> {
    self.consensus_eps.as_ref()
  }

  pub fn reset_consensus_eps(&mut self) {
    self.consensus_eps = None;
  }

  pub fn set_estimated_eps(&mut self, estimated_eps: f32) {
    self.estimated_eps = Some(estimated_eps);
  }

  pub fn with_estimated_eps(mut self, estimated_eps: f32) -> Earning {
    self.estimated_eps = Some(estimated_eps);
    self
  }

  pub fn estimated_eps(&self) -> Option<&f32> {
    self.estimated_eps.as_ref()
  }

  pub fn reset_estimated_eps(&mut self) {
    self.estimated_eps = None;
  }

  pub fn set_announce_time(&mut self, announce_time: String) {
    self.announce_time = Some(announce_time);
  }

  pub fn with_announce_time(mut self, announce_time: String) -> Earning {
    self.announce_time = Some(announce_time);
    self
  }

  pub fn announce_time(&self) -> Option<&String> {
    self.announce_time.as_ref()
  }

  pub fn reset_announce_time(&mut self) {
    self.announce_time = None;
  }

  pub fn set_number_of_estimates(&mut self, number_of_estimates: f32) {
    self.number_of_estimates = Some(number_of_estimates);
  }

  pub fn with_number_of_estimates(mut self, number_of_estimates: f32) -> Earning {
    self.number_of_estimates = Some(number_of_estimates);
    self
  }

  pub fn number_of_estimates(&self) -> Option<&f32> {
    self.number_of_estimates.as_ref()
  }

  pub fn reset_number_of_estimates(&mut self) {
    self.number_of_estimates = None;
  }

  pub fn set_eps_surprise_dollar(&mut self, eps_surprise_dollar: f32) {
    self.eps_surprise_dollar = Some(eps_surprise_dollar);
  }

  pub fn with_eps_surprise_dollar(mut self, eps_surprise_dollar: f32) -> Earning {
    self.eps_surprise_dollar = Some(eps_surprise_dollar);
    self
  }

  pub fn eps_surprise_dollar(&self) -> Option<&f32> {
    self.eps_surprise_dollar.as_ref()
  }

  pub fn reset_eps_surprise_dollar(&mut self) {
    self.eps_surprise_dollar = None;
  }

  pub fn set_year_ago(&mut self, year_ago: f32) {
    self.year_ago = Some(year_ago);
  }

  pub fn with_year_ago(mut self, year_ago: f32) -> Earning {
    self.year_ago = Some(year_ago);
    self
  }

  pub fn year_ago(&self) -> Option<&f32> {
    self.year_ago.as_ref()
  }

  pub fn reset_year_ago(&mut self) {
    self.year_ago = None;
  }

  pub fn set_year_ago_change_percent(&mut self, year_ago_change_percent: f32) {
    self.year_ago_change_percent = Some(year_ago_change_percent);
  }

  pub fn with_year_ago_change_percent(mut self, year_ago_change_percent: f32) -> Earning {
    self.year_ago_change_percent = Some(year_ago_change_percent);
    self
  }

  pub fn year_ago_change_percent(&self) -> Option<&f32> {
    self.year_ago_change_percent.as_ref()
  }

  pub fn reset_year_ago_change_percent(&mut self) {
    self.year_ago_change_percent = None;
  }

  pub fn set_estimated_change_percent(&mut self, estimated_change_percent: f32) {
    self.estimated_change_percent = Some(estimated_change_percent);
  }

  pub fn with_estimated_change_percent(mut self, estimated_change_percent: f32) -> Earning {
    self.estimated_change_percent = Some(estimated_change_percent);
    self
  }

  pub fn estimated_change_percent(&self) -> Option<&f32> {
    self.estimated_change_percent.as_ref()
  }

  pub fn reset_estimated_change_percent(&mut self) {
    self.estimated_change_percent = None;
  }

}


