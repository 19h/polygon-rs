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
pub struct Financial {
  #[serde(rename = "symbol")]
  symbol: String,  // AAPL 
  #[serde(rename = "reportDate")]
  report_date: DateTime<Utc>,  // 2017-12-31T00:00Z 
  #[serde(rename = "reportDateStr")]
  report_date_str: String,  // 2017-12-31 
  #[serde(rename = "grossProfit")]
  gross_profit: Option<f32>,  // 3.3912E+10 
  #[serde(rename = "costOfRevenue")]
  cost_of_revenue: Option<f32>,  // 5.4381E+10 
  #[serde(rename = "operatingRevenue")]
  operating_revenue: Option<f32>,  // 8.8293E+10 
  #[serde(rename = "totalRevenue")]
  total_revenue: Option<f32>,  // 8.8293E+10 
  #[serde(rename = "operatingIncome")]
  operating_income: Option<f32>,  // 2.6274E+10 
  #[serde(rename = "netIncome")]
  net_income: Option<f32>,  // 2.0065E+10 
  #[serde(rename = "researchAndDevelopment")]
  research_and_development: Option<f32>,  // 3.407E+9 
  #[serde(rename = "operatingExpense")]
  operating_expense: Option<f32>,  // 7.638E+9 
  #[serde(rename = "currentAssets")]
  current_assets: Option<f32>,  // 1.4381E+11 
  #[serde(rename = "totalAssets")]
  total_assets: Option<f32>,  // 4.06794E+11 
  #[serde(rename = "totalLiabilities")]
  total_liabilities: Option<f32>,  // 2.66595E+11 
  #[serde(rename = "currentCash")]
  current_cash: Option<f32>,  // 2.7491E+10 
  #[serde(rename = "currentDebt")]
  current_debt: Option<f32>,  // 1.8478E+10 
  #[serde(rename = "totalCash")]
  total_cash: Option<f32>,  // 7.7153E+10 
  #[serde(rename = "totalDebt")]
  total_debt: Option<f32>,  // 1.224E+11 
  #[serde(rename = "shareholderEquity")]
  shareholder_equity: Option<f32>,  // 1.40199E+11 
  #[serde(rename = "cashChange")]
  cash_change: Option<f32>,  // 7.202E+9 
  #[serde(rename = "cashFlow")]
  cash_flow: Option<f32>,  // 2.8293E+10 
  #[serde(rename = "operatingGainsLosses")]
  operating_gains_losses: Option<f32> 
}

impl Financial {
  pub fn new(symbol: String, report_date: DateTime<Utc>, report_date_str: String, ) -> Financial {
    Financial {
      symbol: symbol,
      report_date: report_date,
      report_date_str: report_date_str,
      gross_profit: None,
      cost_of_revenue: None,
      operating_revenue: None,
      total_revenue: None,
      operating_income: None,
      net_income: None,
      research_and_development: None,
      operating_expense: None,
      current_assets: None,
      total_assets: None,
      total_liabilities: None,
      current_cash: None,
      current_debt: None,
      total_cash: None,
      total_debt: None,
      shareholder_equity: None,
      cash_change: None,
      cash_flow: None,
      operating_gains_losses: None
    }
  }

  pub fn set_symbol(&mut self, symbol: String) {
    self.symbol = symbol;
  }

  pub fn with_symbol(mut self, symbol: String) -> Financial {
    self.symbol = symbol;
    self
  }

  pub fn symbol(&self) -> &String {
    &self.symbol
  }


  pub fn set_report_date(&mut self, report_date: DateTime<Utc>) {
    self.report_date = report_date;
  }

  pub fn with_report_date(mut self, report_date: DateTime<Utc>) -> Financial {
    self.report_date = report_date;
    self
  }

  pub fn report_date(&self) -> &DateTime<Utc> {
    &self.report_date
  }


  pub fn set_report_date_str(&mut self, report_date_str: String) {
    self.report_date_str = report_date_str;
  }

  pub fn with_report_date_str(mut self, report_date_str: String) -> Financial {
    self.report_date_str = report_date_str;
    self
  }

  pub fn report_date_str(&self) -> &String {
    &self.report_date_str
  }


  pub fn set_gross_profit(&mut self, gross_profit: f32) {
    self.gross_profit = Some(gross_profit);
  }

  pub fn with_gross_profit(mut self, gross_profit: f32) -> Financial {
    self.gross_profit = Some(gross_profit);
    self
  }

  pub fn gross_profit(&self) -> Option<&f32> {
    self.gross_profit.as_ref()
  }

  pub fn reset_gross_profit(&mut self) {
    self.gross_profit = None;
  }

  pub fn set_cost_of_revenue(&mut self, cost_of_revenue: f32) {
    self.cost_of_revenue = Some(cost_of_revenue);
  }

  pub fn with_cost_of_revenue(mut self, cost_of_revenue: f32) -> Financial {
    self.cost_of_revenue = Some(cost_of_revenue);
    self
  }

  pub fn cost_of_revenue(&self) -> Option<&f32> {
    self.cost_of_revenue.as_ref()
  }

  pub fn reset_cost_of_revenue(&mut self) {
    self.cost_of_revenue = None;
  }

  pub fn set_operating_revenue(&mut self, operating_revenue: f32) {
    self.operating_revenue = Some(operating_revenue);
  }

  pub fn with_operating_revenue(mut self, operating_revenue: f32) -> Financial {
    self.operating_revenue = Some(operating_revenue);
    self
  }

  pub fn operating_revenue(&self) -> Option<&f32> {
    self.operating_revenue.as_ref()
  }

  pub fn reset_operating_revenue(&mut self) {
    self.operating_revenue = None;
  }

  pub fn set_total_revenue(&mut self, total_revenue: f32) {
    self.total_revenue = Some(total_revenue);
  }

  pub fn with_total_revenue(mut self, total_revenue: f32) -> Financial {
    self.total_revenue = Some(total_revenue);
    self
  }

  pub fn total_revenue(&self) -> Option<&f32> {
    self.total_revenue.as_ref()
  }

  pub fn reset_total_revenue(&mut self) {
    self.total_revenue = None;
  }

  pub fn set_operating_income(&mut self, operating_income: f32) {
    self.operating_income = Some(operating_income);
  }

  pub fn with_operating_income(mut self, operating_income: f32) -> Financial {
    self.operating_income = Some(operating_income);
    self
  }

  pub fn operating_income(&self) -> Option<&f32> {
    self.operating_income.as_ref()
  }

  pub fn reset_operating_income(&mut self) {
    self.operating_income = None;
  }

  pub fn set_net_income(&mut self, net_income: f32) {
    self.net_income = Some(net_income);
  }

  pub fn with_net_income(mut self, net_income: f32) -> Financial {
    self.net_income = Some(net_income);
    self
  }

  pub fn net_income(&self) -> Option<&f32> {
    self.net_income.as_ref()
  }

  pub fn reset_net_income(&mut self) {
    self.net_income = None;
  }

  pub fn set_research_and_development(&mut self, research_and_development: f32) {
    self.research_and_development = Some(research_and_development);
  }

  pub fn with_research_and_development(mut self, research_and_development: f32) -> Financial {
    self.research_and_development = Some(research_and_development);
    self
  }

  pub fn research_and_development(&self) -> Option<&f32> {
    self.research_and_development.as_ref()
  }

  pub fn reset_research_and_development(&mut self) {
    self.research_and_development = None;
  }

  pub fn set_operating_expense(&mut self, operating_expense: f32) {
    self.operating_expense = Some(operating_expense);
  }

  pub fn with_operating_expense(mut self, operating_expense: f32) -> Financial {
    self.operating_expense = Some(operating_expense);
    self
  }

  pub fn operating_expense(&self) -> Option<&f32> {
    self.operating_expense.as_ref()
  }

  pub fn reset_operating_expense(&mut self) {
    self.operating_expense = None;
  }

  pub fn set_current_assets(&mut self, current_assets: f32) {
    self.current_assets = Some(current_assets);
  }

  pub fn with_current_assets(mut self, current_assets: f32) -> Financial {
    self.current_assets = Some(current_assets);
    self
  }

  pub fn current_assets(&self) -> Option<&f32> {
    self.current_assets.as_ref()
  }

  pub fn reset_current_assets(&mut self) {
    self.current_assets = None;
  }

  pub fn set_total_assets(&mut self, total_assets: f32) {
    self.total_assets = Some(total_assets);
  }

  pub fn with_total_assets(mut self, total_assets: f32) -> Financial {
    self.total_assets = Some(total_assets);
    self
  }

  pub fn total_assets(&self) -> Option<&f32> {
    self.total_assets.as_ref()
  }

  pub fn reset_total_assets(&mut self) {
    self.total_assets = None;
  }

  pub fn set_total_liabilities(&mut self, total_liabilities: f32) {
    self.total_liabilities = Some(total_liabilities);
  }

  pub fn with_total_liabilities(mut self, total_liabilities: f32) -> Financial {
    self.total_liabilities = Some(total_liabilities);
    self
  }

  pub fn total_liabilities(&self) -> Option<&f32> {
    self.total_liabilities.as_ref()
  }

  pub fn reset_total_liabilities(&mut self) {
    self.total_liabilities = None;
  }

  pub fn set_current_cash(&mut self, current_cash: f32) {
    self.current_cash = Some(current_cash);
  }

  pub fn with_current_cash(mut self, current_cash: f32) -> Financial {
    self.current_cash = Some(current_cash);
    self
  }

  pub fn current_cash(&self) -> Option<&f32> {
    self.current_cash.as_ref()
  }

  pub fn reset_current_cash(&mut self) {
    self.current_cash = None;
  }

  pub fn set_current_debt(&mut self, current_debt: f32) {
    self.current_debt = Some(current_debt);
  }

  pub fn with_current_debt(mut self, current_debt: f32) -> Financial {
    self.current_debt = Some(current_debt);
    self
  }

  pub fn current_debt(&self) -> Option<&f32> {
    self.current_debt.as_ref()
  }

  pub fn reset_current_debt(&mut self) {
    self.current_debt = None;
  }

  pub fn set_total_cash(&mut self, total_cash: f32) {
    self.total_cash = Some(total_cash);
  }

  pub fn with_total_cash(mut self, total_cash: f32) -> Financial {
    self.total_cash = Some(total_cash);
    self
  }

  pub fn total_cash(&self) -> Option<&f32> {
    self.total_cash.as_ref()
  }

  pub fn reset_total_cash(&mut self) {
    self.total_cash = None;
  }

  pub fn set_total_debt(&mut self, total_debt: f32) {
    self.total_debt = Some(total_debt);
  }

  pub fn with_total_debt(mut self, total_debt: f32) -> Financial {
    self.total_debt = Some(total_debt);
    self
  }

  pub fn total_debt(&self) -> Option<&f32> {
    self.total_debt.as_ref()
  }

  pub fn reset_total_debt(&mut self) {
    self.total_debt = None;
  }

  pub fn set_shareholder_equity(&mut self, shareholder_equity: f32) {
    self.shareholder_equity = Some(shareholder_equity);
  }

  pub fn with_shareholder_equity(mut self, shareholder_equity: f32) -> Financial {
    self.shareholder_equity = Some(shareholder_equity);
    self
  }

  pub fn shareholder_equity(&self) -> Option<&f32> {
    self.shareholder_equity.as_ref()
  }

  pub fn reset_shareholder_equity(&mut self) {
    self.shareholder_equity = None;
  }

  pub fn set_cash_change(&mut self, cash_change: f32) {
    self.cash_change = Some(cash_change);
  }

  pub fn with_cash_change(mut self, cash_change: f32) -> Financial {
    self.cash_change = Some(cash_change);
    self
  }

  pub fn cash_change(&self) -> Option<&f32> {
    self.cash_change.as_ref()
  }

  pub fn reset_cash_change(&mut self) {
    self.cash_change = None;
  }

  pub fn set_cash_flow(&mut self, cash_flow: f32) {
    self.cash_flow = Some(cash_flow);
  }

  pub fn with_cash_flow(mut self, cash_flow: f32) -> Financial {
    self.cash_flow = Some(cash_flow);
    self
  }

  pub fn cash_flow(&self) -> Option<&f32> {
    self.cash_flow.as_ref()
  }

  pub fn reset_cash_flow(&mut self) {
    self.cash_flow = None;
  }

  pub fn set_operating_gains_losses(&mut self, operating_gains_losses: f32) {
    self.operating_gains_losses = Some(operating_gains_losses);
  }

  pub fn with_operating_gains_losses(mut self, operating_gains_losses: f32) -> Financial {
    self.operating_gains_losses = Some(operating_gains_losses);
    self
  }

  pub fn operating_gains_losses(&self) -> Option<&f32> {
    self.operating_gains_losses.as_ref()
  }

  pub fn reset_operating_gains_losses(&mut self) {
    self.operating_gains_losses = None;
  }

}


