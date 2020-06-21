/* 
 * Polygon API
 *
 * The future of fintech.
 *
 * OpenAPI spec version: 1.0.1

 */
#![allow(unused_imports)]
/// Financials : Financials

use serde_json::Value;
use bigdecimal::BigDecimal;
use chrono::{NaiveDateTime, DateTime, FixedOffset, Utc};

use crate::models::*;
//use crate::date_serializer;
//use crate::datetime_serializer;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Financials {
  #[serde(rename = "ticker")]
  ticker: TickerSymbol, 
  #[serde(rename = "period")]
  period: Option<String>,  // Q 
  #[serde(rename = "calendarDate")]
  calendar_date: Option<DateTime<Utc>>, 
  #[serde(rename = "reportPeriod")]
  report_period: Option<DateTime<Utc>>, 
  #[serde(rename = "updated")]
  updated: Option<DateTime<Utc>>, 
  #[serde(rename = "accumulatedOtherComprehensiveIncome")]
  accumulated_other_comprehensive_income: Option<i64>, 
  #[serde(rename = "assets")]
  assets: Option<i64>, 
  #[serde(rename = "assetsAverage")]
  assets_average: Option<i64>, 
  #[serde(rename = "assetsCurrent")]
  assets_current: Option<i64>, 
  #[serde(rename = "assetTurnover")]
  asset_turnover: Option<i64>, 
  #[serde(rename = "assetsNonCurrent")]
  assets_non_current: Option<i64>, 
  #[serde(rename = "bookValuePerShare")]
  book_value_per_share: Option<i64>, 
  #[serde(rename = "capitalExpenditure")]
  capital_expenditure: Option<i64>, 
  #[serde(rename = "cashAndEquivalents")]
  cash_and_equivalents: Option<i64>, 
  #[serde(rename = "cashAndEquivalentsUSD")]
  cash_and_equivalents_usd: Option<i64>, 
  #[serde(rename = "costOfRevenue")]
  cost_of_revenue: Option<i64>, 
  #[serde(rename = "consolidatedIncome")]
  consolidated_income: Option<i64>, 
  #[serde(rename = "currentRatio")]
  current_ratio: Option<i64>, 
  #[serde(rename = "debtToEquityRatio")]
  debt_to_equity_ratio: Option<i64>, 
  #[serde(rename = "debt")]
  debt: Option<i64>, 
  #[serde(rename = "debtCurrent")]
  debt_current: Option<i64>, 
  #[serde(rename = "debtNonCurrent")]
  debt_non_current: Option<i64>, 
  #[serde(rename = "debtUSD")]
  debt_usd: Option<i64>, 
  #[serde(rename = "deferredRevenue")]
  deferred_revenue: Option<i64>, 
  #[serde(rename = "depreciationAmortizationAndAccretion")]
  depreciation_amortization_and_accretion: Option<i64>, 
  #[serde(rename = "deposits")]
  deposits: Option<i64>, 
  #[serde(rename = "dividendYield")]
  dividend_yield: Option<i64>, 
  #[serde(rename = "dividendsPerBasicCommonShare")]
  dividends_per_basic_common_share: Option<i64>, 
  #[serde(rename = "earningBeforeInterestTaxes")]
  earning_before_interest_taxes: Option<i64>, 
  #[serde(rename = "earningsBeforeInterestTaxesDepreciationAmortization")]
  earnings_before_interest_taxes_depreciation_amortization: Option<i64>, 
  #[serde(rename = "EBITDAMargin")]
  ebitda_margin: Option<i64>, 
  #[serde(rename = "earningsBeforeInterestTaxesDepreciationAmortizationUSD")]
  earnings_before_interest_taxes_depreciation_amortization_usd: Option<i64>, 
  #[serde(rename = "earningBeforeInterestTaxesUSD")]
  earning_before_interest_taxes_usd: Option<i64>, 
  #[serde(rename = "earningsBeforeTax")]
  earnings_before_tax: Option<i64>, 
  #[serde(rename = "earningsPerBasicShare")]
  earnings_per_basic_share: Option<i64>, 
  #[serde(rename = "earningsPerDilutedShare")]
  earnings_per_diluted_share: Option<i64>, 
  #[serde(rename = "earningsPerBasicShareUSD")]
  earnings_per_basic_share_usd: Option<i64>, 
  #[serde(rename = "shareholdersEquity")]
  shareholders_equity: Option<i64>, 
  #[serde(rename = "averageEquity")]
  average_equity: Option<i64>, 
  #[serde(rename = "shareholdersEquityUSD")]
  shareholders_equity_usd: Option<i64>, 
  #[serde(rename = "enterpriseValue")]
  enterprise_value: Option<i64>, 
  #[serde(rename = "enterpriseValueOverEBIT")]
  enterprise_value_over_ebit: Option<i64>, 
  #[serde(rename = "enterpriseValueOverEBITDA")]
  enterprise_value_over_ebitda: Option<i64>, 
  #[serde(rename = "freeCashFlow")]
  free_cash_flow: Option<i64>, 
  #[serde(rename = "freeCashFlowPerShare")]
  free_cash_flow_per_share: Option<i64>, 
  #[serde(rename = "foreignCurrencyUSDExchangeRate")]
  foreign_currency_usd_exchange_rate: Option<i64>, 
  #[serde(rename = "grossProfit")]
  gross_profit: Option<i64>, 
  #[serde(rename = "grossMargin")]
  gross_margin: Option<i64>, 
  #[serde(rename = "goodwillAndIntangibleAssets")]
  goodwill_and_intangible_assets: Option<i64>, 
  #[serde(rename = "interestExpense")]
  interest_expense: Option<i64>, 
  #[serde(rename = "investedCapital")]
  invested_capital: Option<i64>, 
  #[serde(rename = "investedCapitalAverage")]
  invested_capital_average: Option<i64>, 
  #[serde(rename = "inventory")]
  inventory: Option<i64>, 
  #[serde(rename = "investments")]
  investments: Option<i64>, 
  #[serde(rename = "investmentsCurrent")]
  investments_current: Option<i64>, 
  #[serde(rename = "investmentsNonCurrent")]
  investments_non_current: Option<i64>, 
  #[serde(rename = "totalLiabilities")]
  total_liabilities: Option<i64>, 
  #[serde(rename = "currentLiabilities")]
  current_liabilities: Option<i64>, 
  #[serde(rename = "liabilitiesNonCurrent")]
  liabilities_non_current: Option<i64>, 
  #[serde(rename = "marketCapitalization")]
  market_capitalization: Option<i64>, 
  #[serde(rename = "netCashFlow")]
  net_cash_flow: Option<i64>, 
  #[serde(rename = "netCashFlowBusinessAcquisitionsDisposals")]
  net_cash_flow_business_acquisitions_disposals: Option<i64>, 
  #[serde(rename = "issuanceEquityShares")]
  issuance_equity_shares: Option<i64>, 
  #[serde(rename = "issuanceDebtSecurities")]
  issuance_debt_securities: Option<i64>, 
  #[serde(rename = "paymentDividendsOtherCashDistributions")]
  payment_dividends_other_cash_distributions: Option<i64>, 
  #[serde(rename = "netCashFlowFromFinancing")]
  net_cash_flow_from_financing: Option<i64>, 
  #[serde(rename = "netCashFlowFromInvesting")]
  net_cash_flow_from_investing: Option<i64>, 
  #[serde(rename = "netCashFlowInvestmentAcquisitionsDisposals")]
  net_cash_flow_investment_acquisitions_disposals: Option<i64>, 
  #[serde(rename = "netCashFlowFromOperations")]
  net_cash_flow_from_operations: Option<i64>, 
  #[serde(rename = "effectOfExchangeRateChangesOnCash")]
  effect_of_exchange_rate_changes_on_cash: Option<i64>, 
  #[serde(rename = "netIncome")]
  net_income: Option<i64>, 
  #[serde(rename = "netIncomeCommonStock")]
  net_income_common_stock: Option<i64>, 
  #[serde(rename = "netIncomeCommonStockUSD")]
  net_income_common_stock_usd: Option<i64>, 
  #[serde(rename = "netLossIncomeFromDiscontinuedOperations")]
  net_loss_income_from_discontinued_operations: Option<i64>, 
  #[serde(rename = "netIncomeToNonControllingInterests")]
  net_income_to_non_controlling_interests: Option<i64>, 
  #[serde(rename = "profitMargin")]
  profit_margin: Option<i64>, 
  #[serde(rename = "operatingExpenses")]
  operating_expenses: Option<i64>, 
  #[serde(rename = "operatingIncome")]
  operating_income: Option<i64>, 
  #[serde(rename = "tradeAndNonTradePayables")]
  trade_and_non_trade_payables: Option<i64>, 
  #[serde(rename = "payoutRatio")]
  payout_ratio: Option<i64>, 
  #[serde(rename = "priceToBookValue")]
  price_to_book_value: Option<i64>, 
  #[serde(rename = "priceEarnings")]
  price_earnings: Option<i64>, 
  #[serde(rename = "priceToEarningsRatio")]
  price_to_earnings_ratio: Option<i64>, 
  #[serde(rename = "propertyPlantEquipmentNet")]
  property_plant_equipment_net: Option<i64>, 
  #[serde(rename = "preferredDividendsIncomeStatementImpact")]
  preferred_dividends_income_statement_impact: Option<i64>, 
  #[serde(rename = "sharePriceAdjustedClose")]
  share_price_adjusted_close: Option<i64>, 
  #[serde(rename = "priceSales")]
  price_sales: Option<i64>, 
  #[serde(rename = "priceToSalesRatio")]
  price_to_sales_ratio: Option<i64>, 
  #[serde(rename = "tradeAndNonTradeReceivables")]
  trade_and_non_trade_receivables: Option<i64>, 
  #[serde(rename = "accumulatedRetainedEarningsDeficit")]
  accumulated_retained_earnings_deficit: Option<i64>, 
  #[serde(rename = "revenues")]
  revenues: Option<i64>, 
  #[serde(rename = "revenuesUSD")]
  revenues_usd: Option<i64>, 
  #[serde(rename = "researchAndDevelopmentExpense")]
  research_and_development_expense: Option<i64>, 
  #[serde(rename = "returnOnAverageAssets")]
  return_on_average_assets: Option<i64>, 
  #[serde(rename = "returnOnAverageEquity")]
  return_on_average_equity: Option<i64>, 
  #[serde(rename = "returnOnInvestedCapital")]
  return_on_invested_capital: Option<i64>, 
  #[serde(rename = "returnOnSales")]
  return_on_sales: Option<i64>, 
  #[serde(rename = "shareBasedCompensation")]
  share_based_compensation: Option<i64>, 
  #[serde(rename = "sellingGeneralAndAdministrativeExpense")]
  selling_general_and_administrative_expense: Option<i64>, 
  #[serde(rename = "shareFactor")]
  share_factor: Option<i64>, 
  #[serde(rename = "shares")]
  shares: Option<i64>, 
  #[serde(rename = "weightedAverageShares")]
  weighted_average_shares: Option<i64>, 
  #[serde(rename = "weightedAverageSharesDiluted")]
  weighted_average_shares_diluted: Option<i64>, 
  #[serde(rename = "salesPerShare")]
  sales_per_share: Option<i64>, 
  #[serde(rename = "tangibleAssetValue")]
  tangible_asset_value: Option<i64>, 
  #[serde(rename = "taxAssets")]
  tax_assets: Option<i64>, 
  #[serde(rename = "incomeTaxExpense")]
  income_tax_expense: Option<i64>, 
  #[serde(rename = "taxLiabilities")]
  tax_liabilities: Option<i64>, 
  #[serde(rename = "tangibleAssetsBookValuePerShare")]
  tangible_assets_book_value_per_share: Option<i64>, 
  #[serde(rename = "workingCapital")]
  working_capital: Option<i64> 
}

impl Financials {
  pub fn new(ticker: TickerSymbol, ) -> Financials {
    Financials {
      ticker: ticker,
      period: None,
      calendar_date: None,
      report_period: None,
      updated: None,
      accumulated_other_comprehensive_income: None,
      assets: None,
      assets_average: None,
      assets_current: None,
      asset_turnover: None,
      assets_non_current: None,
      book_value_per_share: None,
      capital_expenditure: None,
      cash_and_equivalents: None,
      cash_and_equivalents_usd: None,
      cost_of_revenue: None,
      consolidated_income: None,
      current_ratio: None,
      debt_to_equity_ratio: None,
      debt: None,
      debt_current: None,
      debt_non_current: None,
      debt_usd: None,
      deferred_revenue: None,
      depreciation_amortization_and_accretion: None,
      deposits: None,
      dividend_yield: None,
      dividends_per_basic_common_share: None,
      earning_before_interest_taxes: None,
      earnings_before_interest_taxes_depreciation_amortization: None,
      ebitda_margin: None,
      earnings_before_interest_taxes_depreciation_amortization_usd: None,
      earning_before_interest_taxes_usd: None,
      earnings_before_tax: None,
      earnings_per_basic_share: None,
      earnings_per_diluted_share: None,
      earnings_per_basic_share_usd: None,
      shareholders_equity: None,
      average_equity: None,
      shareholders_equity_usd: None,
      enterprise_value: None,
      enterprise_value_over_ebit: None,
      enterprise_value_over_ebitda: None,
      free_cash_flow: None,
      free_cash_flow_per_share: None,
      foreign_currency_usd_exchange_rate: None,
      gross_profit: None,
      gross_margin: None,
      goodwill_and_intangible_assets: None,
      interest_expense: None,
      invested_capital: None,
      invested_capital_average: None,
      inventory: None,
      investments: None,
      investments_current: None,
      investments_non_current: None,
      total_liabilities: None,
      current_liabilities: None,
      liabilities_non_current: None,
      market_capitalization: None,
      net_cash_flow: None,
      net_cash_flow_business_acquisitions_disposals: None,
      issuance_equity_shares: None,
      issuance_debt_securities: None,
      payment_dividends_other_cash_distributions: None,
      net_cash_flow_from_financing: None,
      net_cash_flow_from_investing: None,
      net_cash_flow_investment_acquisitions_disposals: None,
      net_cash_flow_from_operations: None,
      effect_of_exchange_rate_changes_on_cash: None,
      net_income: None,
      net_income_common_stock: None,
      net_income_common_stock_usd: None,
      net_loss_income_from_discontinued_operations: None,
      net_income_to_non_controlling_interests: None,
      profit_margin: None,
      operating_expenses: None,
      operating_income: None,
      trade_and_non_trade_payables: None,
      payout_ratio: None,
      price_to_book_value: None,
      price_earnings: None,
      price_to_earnings_ratio: None,
      property_plant_equipment_net: None,
      preferred_dividends_income_statement_impact: None,
      share_price_adjusted_close: None,
      price_sales: None,
      price_to_sales_ratio: None,
      trade_and_non_trade_receivables: None,
      accumulated_retained_earnings_deficit: None,
      revenues: None,
      revenues_usd: None,
      research_and_development_expense: None,
      return_on_average_assets: None,
      return_on_average_equity: None,
      return_on_invested_capital: None,
      return_on_sales: None,
      share_based_compensation: None,
      selling_general_and_administrative_expense: None,
      share_factor: None,
      shares: None,
      weighted_average_shares: None,
      weighted_average_shares_diluted: None,
      sales_per_share: None,
      tangible_asset_value: None,
      tax_assets: None,
      income_tax_expense: None,
      tax_liabilities: None,
      tangible_assets_book_value_per_share: None,
      working_capital: None
    }
  }

  pub fn set_ticker(&mut self, ticker: TickerSymbol) {
    self.ticker = ticker;
  }

  pub fn with_ticker(mut self, ticker: TickerSymbol) -> Financials {
    self.ticker = ticker;
    self
  }

  pub fn ticker(&self) -> &TickerSymbol {
    &self.ticker
  }


  pub fn set_period(&mut self, period: String) {
    self.period = Some(period);
  }

  pub fn with_period(mut self, period: String) -> Financials {
    self.period = Some(period);
    self
  }

  pub fn period(&self) -> Option<&String> {
    self.period.as_ref()
  }

  pub fn reset_period(&mut self) {
    self.period = None;
  }

  pub fn set_calendar_date(&mut self, calendar_date: DateTime<Utc>) {
    self.calendar_date = Some(calendar_date);
  }

  pub fn with_calendar_date(mut self, calendar_date: DateTime<Utc>) -> Financials {
    self.calendar_date = Some(calendar_date);
    self
  }

  pub fn calendar_date(&self) -> Option<&DateTime<Utc>> {
    self.calendar_date.as_ref()
  }

  pub fn reset_calendar_date(&mut self) {
    self.calendar_date = None;
  }

  pub fn set_report_period(&mut self, report_period: DateTime<Utc>) {
    self.report_period = Some(report_period);
  }

  pub fn with_report_period(mut self, report_period: DateTime<Utc>) -> Financials {
    self.report_period = Some(report_period);
    self
  }

  pub fn report_period(&self) -> Option<&DateTime<Utc>> {
    self.report_period.as_ref()
  }

  pub fn reset_report_period(&mut self) {
    self.report_period = None;
  }

  pub fn set_updated(&mut self, updated: DateTime<Utc>) {
    self.updated = Some(updated);
  }

  pub fn with_updated(mut self, updated: DateTime<Utc>) -> Financials {
    self.updated = Some(updated);
    self
  }

  pub fn updated(&self) -> Option<&DateTime<Utc>> {
    self.updated.as_ref()
  }

  pub fn reset_updated(&mut self) {
    self.updated = None;
  }

  pub fn set_accumulated_other_comprehensive_income(&mut self, accumulated_other_comprehensive_income: i64) {
    self.accumulated_other_comprehensive_income = Some(accumulated_other_comprehensive_income);
  }

  pub fn with_accumulated_other_comprehensive_income(mut self, accumulated_other_comprehensive_income: i64) -> Financials {
    self.accumulated_other_comprehensive_income = Some(accumulated_other_comprehensive_income);
    self
  }

  pub fn accumulated_other_comprehensive_income(&self) -> Option<&i64> {
    self.accumulated_other_comprehensive_income.as_ref()
  }

  pub fn reset_accumulated_other_comprehensive_income(&mut self) {
    self.accumulated_other_comprehensive_income = None;
  }

  pub fn set_assets(&mut self, assets: i64) {
    self.assets = Some(assets);
  }

  pub fn with_assets(mut self, assets: i64) -> Financials {
    self.assets = Some(assets);
    self
  }

  pub fn assets(&self) -> Option<&i64> {
    self.assets.as_ref()
  }

  pub fn reset_assets(&mut self) {
    self.assets = None;
  }

  pub fn set_assets_average(&mut self, assets_average: i64) {
    self.assets_average = Some(assets_average);
  }

  pub fn with_assets_average(mut self, assets_average: i64) -> Financials {
    self.assets_average = Some(assets_average);
    self
  }

  pub fn assets_average(&self) -> Option<&i64> {
    self.assets_average.as_ref()
  }

  pub fn reset_assets_average(&mut self) {
    self.assets_average = None;
  }

  pub fn set_assets_current(&mut self, assets_current: i64) {
    self.assets_current = Some(assets_current);
  }

  pub fn with_assets_current(mut self, assets_current: i64) -> Financials {
    self.assets_current = Some(assets_current);
    self
  }

  pub fn assets_current(&self) -> Option<&i64> {
    self.assets_current.as_ref()
  }

  pub fn reset_assets_current(&mut self) {
    self.assets_current = None;
  }

  pub fn set_asset_turnover(&mut self, asset_turnover: i64) {
    self.asset_turnover = Some(asset_turnover);
  }

  pub fn with_asset_turnover(mut self, asset_turnover: i64) -> Financials {
    self.asset_turnover = Some(asset_turnover);
    self
  }

  pub fn asset_turnover(&self) -> Option<&i64> {
    self.asset_turnover.as_ref()
  }

  pub fn reset_asset_turnover(&mut self) {
    self.asset_turnover = None;
  }

  pub fn set_assets_non_current(&mut self, assets_non_current: i64) {
    self.assets_non_current = Some(assets_non_current);
  }

  pub fn with_assets_non_current(mut self, assets_non_current: i64) -> Financials {
    self.assets_non_current = Some(assets_non_current);
    self
  }

  pub fn assets_non_current(&self) -> Option<&i64> {
    self.assets_non_current.as_ref()
  }

  pub fn reset_assets_non_current(&mut self) {
    self.assets_non_current = None;
  }

  pub fn set_book_value_per_share(&mut self, book_value_per_share: i64) {
    self.book_value_per_share = Some(book_value_per_share);
  }

  pub fn with_book_value_per_share(mut self, book_value_per_share: i64) -> Financials {
    self.book_value_per_share = Some(book_value_per_share);
    self
  }

  pub fn book_value_per_share(&self) -> Option<&i64> {
    self.book_value_per_share.as_ref()
  }

  pub fn reset_book_value_per_share(&mut self) {
    self.book_value_per_share = None;
  }

  pub fn set_capital_expenditure(&mut self, capital_expenditure: i64) {
    self.capital_expenditure = Some(capital_expenditure);
  }

  pub fn with_capital_expenditure(mut self, capital_expenditure: i64) -> Financials {
    self.capital_expenditure = Some(capital_expenditure);
    self
  }

  pub fn capital_expenditure(&self) -> Option<&i64> {
    self.capital_expenditure.as_ref()
  }

  pub fn reset_capital_expenditure(&mut self) {
    self.capital_expenditure = None;
  }

  pub fn set_cash_and_equivalents(&mut self, cash_and_equivalents: i64) {
    self.cash_and_equivalents = Some(cash_and_equivalents);
  }

  pub fn with_cash_and_equivalents(mut self, cash_and_equivalents: i64) -> Financials {
    self.cash_and_equivalents = Some(cash_and_equivalents);
    self
  }

  pub fn cash_and_equivalents(&self) -> Option<&i64> {
    self.cash_and_equivalents.as_ref()
  }

  pub fn reset_cash_and_equivalents(&mut self) {
    self.cash_and_equivalents = None;
  }

  pub fn set_cash_and_equivalents_usd(&mut self, cash_and_equivalents_usd: i64) {
    self.cash_and_equivalents_usd = Some(cash_and_equivalents_usd);
  }

  pub fn with_cash_and_equivalents_usd(mut self, cash_and_equivalents_usd: i64) -> Financials {
    self.cash_and_equivalents_usd = Some(cash_and_equivalents_usd);
    self
  }

  pub fn cash_and_equivalents_usd(&self) -> Option<&i64> {
    self.cash_and_equivalents_usd.as_ref()
  }

  pub fn reset_cash_and_equivalents_usd(&mut self) {
    self.cash_and_equivalents_usd = None;
  }

  pub fn set_cost_of_revenue(&mut self, cost_of_revenue: i64) {
    self.cost_of_revenue = Some(cost_of_revenue);
  }

  pub fn with_cost_of_revenue(mut self, cost_of_revenue: i64) -> Financials {
    self.cost_of_revenue = Some(cost_of_revenue);
    self
  }

  pub fn cost_of_revenue(&self) -> Option<&i64> {
    self.cost_of_revenue.as_ref()
  }

  pub fn reset_cost_of_revenue(&mut self) {
    self.cost_of_revenue = None;
  }

  pub fn set_consolidated_income(&mut self, consolidated_income: i64) {
    self.consolidated_income = Some(consolidated_income);
  }

  pub fn with_consolidated_income(mut self, consolidated_income: i64) -> Financials {
    self.consolidated_income = Some(consolidated_income);
    self
  }

  pub fn consolidated_income(&self) -> Option<&i64> {
    self.consolidated_income.as_ref()
  }

  pub fn reset_consolidated_income(&mut self) {
    self.consolidated_income = None;
  }

  pub fn set_current_ratio(&mut self, current_ratio: i64) {
    self.current_ratio = Some(current_ratio);
  }

  pub fn with_current_ratio(mut self, current_ratio: i64) -> Financials {
    self.current_ratio = Some(current_ratio);
    self
  }

  pub fn current_ratio(&self) -> Option<&i64> {
    self.current_ratio.as_ref()
  }

  pub fn reset_current_ratio(&mut self) {
    self.current_ratio = None;
  }

  pub fn set_debt_to_equity_ratio(&mut self, debt_to_equity_ratio: i64) {
    self.debt_to_equity_ratio = Some(debt_to_equity_ratio);
  }

  pub fn with_debt_to_equity_ratio(mut self, debt_to_equity_ratio: i64) -> Financials {
    self.debt_to_equity_ratio = Some(debt_to_equity_ratio);
    self
  }

  pub fn debt_to_equity_ratio(&self) -> Option<&i64> {
    self.debt_to_equity_ratio.as_ref()
  }

  pub fn reset_debt_to_equity_ratio(&mut self) {
    self.debt_to_equity_ratio = None;
  }

  pub fn set_debt(&mut self, debt: i64) {
    self.debt = Some(debt);
  }

  pub fn with_debt(mut self, debt: i64) -> Financials {
    self.debt = Some(debt);
    self
  }

  pub fn debt(&self) -> Option<&i64> {
    self.debt.as_ref()
  }

  pub fn reset_debt(&mut self) {
    self.debt = None;
  }

  pub fn set_debt_current(&mut self, debt_current: i64) {
    self.debt_current = Some(debt_current);
  }

  pub fn with_debt_current(mut self, debt_current: i64) -> Financials {
    self.debt_current = Some(debt_current);
    self
  }

  pub fn debt_current(&self) -> Option<&i64> {
    self.debt_current.as_ref()
  }

  pub fn reset_debt_current(&mut self) {
    self.debt_current = None;
  }

  pub fn set_debt_non_current(&mut self, debt_non_current: i64) {
    self.debt_non_current = Some(debt_non_current);
  }

  pub fn with_debt_non_current(mut self, debt_non_current: i64) -> Financials {
    self.debt_non_current = Some(debt_non_current);
    self
  }

  pub fn debt_non_current(&self) -> Option<&i64> {
    self.debt_non_current.as_ref()
  }

  pub fn reset_debt_non_current(&mut self) {
    self.debt_non_current = None;
  }

  pub fn set_debt_usd(&mut self, debt_usd: i64) {
    self.debt_usd = Some(debt_usd);
  }

  pub fn with_debt_usd(mut self, debt_usd: i64) -> Financials {
    self.debt_usd = Some(debt_usd);
    self
  }

  pub fn debt_usd(&self) -> Option<&i64> {
    self.debt_usd.as_ref()
  }

  pub fn reset_debt_usd(&mut self) {
    self.debt_usd = None;
  }

  pub fn set_deferred_revenue(&mut self, deferred_revenue: i64) {
    self.deferred_revenue = Some(deferred_revenue);
  }

  pub fn with_deferred_revenue(mut self, deferred_revenue: i64) -> Financials {
    self.deferred_revenue = Some(deferred_revenue);
    self
  }

  pub fn deferred_revenue(&self) -> Option<&i64> {
    self.deferred_revenue.as_ref()
  }

  pub fn reset_deferred_revenue(&mut self) {
    self.deferred_revenue = None;
  }

  pub fn set_depreciation_amortization_and_accretion(&mut self, depreciation_amortization_and_accretion: i64) {
    self.depreciation_amortization_and_accretion = Some(depreciation_amortization_and_accretion);
  }

  pub fn with_depreciation_amortization_and_accretion(mut self, depreciation_amortization_and_accretion: i64) -> Financials {
    self.depreciation_amortization_and_accretion = Some(depreciation_amortization_and_accretion);
    self
  }

  pub fn depreciation_amortization_and_accretion(&self) -> Option<&i64> {
    self.depreciation_amortization_and_accretion.as_ref()
  }

  pub fn reset_depreciation_amortization_and_accretion(&mut self) {
    self.depreciation_amortization_and_accretion = None;
  }

  pub fn set_deposits(&mut self, deposits: i64) {
    self.deposits = Some(deposits);
  }

  pub fn with_deposits(mut self, deposits: i64) -> Financials {
    self.deposits = Some(deposits);
    self
  }

  pub fn deposits(&self) -> Option<&i64> {
    self.deposits.as_ref()
  }

  pub fn reset_deposits(&mut self) {
    self.deposits = None;
  }

  pub fn set_dividend_yield(&mut self, dividend_yield: i64) {
    self.dividend_yield = Some(dividend_yield);
  }

  pub fn with_dividend_yield(mut self, dividend_yield: i64) -> Financials {
    self.dividend_yield = Some(dividend_yield);
    self
  }

  pub fn dividend_yield(&self) -> Option<&i64> {
    self.dividend_yield.as_ref()
  }

  pub fn reset_dividend_yield(&mut self) {
    self.dividend_yield = None;
  }

  pub fn set_dividends_per_basic_common_share(&mut self, dividends_per_basic_common_share: i64) {
    self.dividends_per_basic_common_share = Some(dividends_per_basic_common_share);
  }

  pub fn with_dividends_per_basic_common_share(mut self, dividends_per_basic_common_share: i64) -> Financials {
    self.dividends_per_basic_common_share = Some(dividends_per_basic_common_share);
    self
  }

  pub fn dividends_per_basic_common_share(&self) -> Option<&i64> {
    self.dividends_per_basic_common_share.as_ref()
  }

  pub fn reset_dividends_per_basic_common_share(&mut self) {
    self.dividends_per_basic_common_share = None;
  }

  pub fn set_earning_before_interest_taxes(&mut self, earning_before_interest_taxes: i64) {
    self.earning_before_interest_taxes = Some(earning_before_interest_taxes);
  }

  pub fn with_earning_before_interest_taxes(mut self, earning_before_interest_taxes: i64) -> Financials {
    self.earning_before_interest_taxes = Some(earning_before_interest_taxes);
    self
  }

  pub fn earning_before_interest_taxes(&self) -> Option<&i64> {
    self.earning_before_interest_taxes.as_ref()
  }

  pub fn reset_earning_before_interest_taxes(&mut self) {
    self.earning_before_interest_taxes = None;
  }

  pub fn set_earnings_before_interest_taxes_depreciation_amortization(&mut self, earnings_before_interest_taxes_depreciation_amortization: i64) {
    self.earnings_before_interest_taxes_depreciation_amortization = Some(earnings_before_interest_taxes_depreciation_amortization);
  }

  pub fn with_earnings_before_interest_taxes_depreciation_amortization(mut self, earnings_before_interest_taxes_depreciation_amortization: i64) -> Financials {
    self.earnings_before_interest_taxes_depreciation_amortization = Some(earnings_before_interest_taxes_depreciation_amortization);
    self
  }

  pub fn earnings_before_interest_taxes_depreciation_amortization(&self) -> Option<&i64> {
    self.earnings_before_interest_taxes_depreciation_amortization.as_ref()
  }

  pub fn reset_earnings_before_interest_taxes_depreciation_amortization(&mut self) {
    self.earnings_before_interest_taxes_depreciation_amortization = None;
  }

  pub fn set_ebitda_margin(&mut self, ebitda_margin: i64) {
    self.ebitda_margin = Some(ebitda_margin);
  }

  pub fn with_ebitda_margin(mut self, ebitda_margin: i64) -> Financials {
    self.ebitda_margin = Some(ebitda_margin);
    self
  }

  pub fn ebitda_margin(&self) -> Option<&i64> {
    self.ebitda_margin.as_ref()
  }

  pub fn reset_ebitda_margin(&mut self) {
    self.ebitda_margin = None;
  }

  pub fn set_earnings_before_interest_taxes_depreciation_amortization_usd(&mut self, earnings_before_interest_taxes_depreciation_amortization_usd: i64) {
    self.earnings_before_interest_taxes_depreciation_amortization_usd = Some(earnings_before_interest_taxes_depreciation_amortization_usd);
  }

  pub fn with_earnings_before_interest_taxes_depreciation_amortization_usd(mut self, earnings_before_interest_taxes_depreciation_amortization_usd: i64) -> Financials {
    self.earnings_before_interest_taxes_depreciation_amortization_usd = Some(earnings_before_interest_taxes_depreciation_amortization_usd);
    self
  }

  pub fn earnings_before_interest_taxes_depreciation_amortization_usd(&self) -> Option<&i64> {
    self.earnings_before_interest_taxes_depreciation_amortization_usd.as_ref()
  }

  pub fn reset_earnings_before_interest_taxes_depreciation_amortization_usd(&mut self) {
    self.earnings_before_interest_taxes_depreciation_amortization_usd = None;
  }

  pub fn set_earning_before_interest_taxes_usd(&mut self, earning_before_interest_taxes_usd: i64) {
    self.earning_before_interest_taxes_usd = Some(earning_before_interest_taxes_usd);
  }

  pub fn with_earning_before_interest_taxes_usd(mut self, earning_before_interest_taxes_usd: i64) -> Financials {
    self.earning_before_interest_taxes_usd = Some(earning_before_interest_taxes_usd);
    self
  }

  pub fn earning_before_interest_taxes_usd(&self) -> Option<&i64> {
    self.earning_before_interest_taxes_usd.as_ref()
  }

  pub fn reset_earning_before_interest_taxes_usd(&mut self) {
    self.earning_before_interest_taxes_usd = None;
  }

  pub fn set_earnings_before_tax(&mut self, earnings_before_tax: i64) {
    self.earnings_before_tax = Some(earnings_before_tax);
  }

  pub fn with_earnings_before_tax(mut self, earnings_before_tax: i64) -> Financials {
    self.earnings_before_tax = Some(earnings_before_tax);
    self
  }

  pub fn earnings_before_tax(&self) -> Option<&i64> {
    self.earnings_before_tax.as_ref()
  }

  pub fn reset_earnings_before_tax(&mut self) {
    self.earnings_before_tax = None;
  }

  pub fn set_earnings_per_basic_share(&mut self, earnings_per_basic_share: i64) {
    self.earnings_per_basic_share = Some(earnings_per_basic_share);
  }

  pub fn with_earnings_per_basic_share(mut self, earnings_per_basic_share: i64) -> Financials {
    self.earnings_per_basic_share = Some(earnings_per_basic_share);
    self
  }

  pub fn earnings_per_basic_share(&self) -> Option<&i64> {
    self.earnings_per_basic_share.as_ref()
  }

  pub fn reset_earnings_per_basic_share(&mut self) {
    self.earnings_per_basic_share = None;
  }

  pub fn set_earnings_per_diluted_share(&mut self, earnings_per_diluted_share: i64) {
    self.earnings_per_diluted_share = Some(earnings_per_diluted_share);
  }

  pub fn with_earnings_per_diluted_share(mut self, earnings_per_diluted_share: i64) -> Financials {
    self.earnings_per_diluted_share = Some(earnings_per_diluted_share);
    self
  }

  pub fn earnings_per_diluted_share(&self) -> Option<&i64> {
    self.earnings_per_diluted_share.as_ref()
  }

  pub fn reset_earnings_per_diluted_share(&mut self) {
    self.earnings_per_diluted_share = None;
  }

  pub fn set_earnings_per_basic_share_usd(&mut self, earnings_per_basic_share_usd: i64) {
    self.earnings_per_basic_share_usd = Some(earnings_per_basic_share_usd);
  }

  pub fn with_earnings_per_basic_share_usd(mut self, earnings_per_basic_share_usd: i64) -> Financials {
    self.earnings_per_basic_share_usd = Some(earnings_per_basic_share_usd);
    self
  }

  pub fn earnings_per_basic_share_usd(&self) -> Option<&i64> {
    self.earnings_per_basic_share_usd.as_ref()
  }

  pub fn reset_earnings_per_basic_share_usd(&mut self) {
    self.earnings_per_basic_share_usd = None;
  }

  pub fn set_shareholders_equity(&mut self, shareholders_equity: i64) {
    self.shareholders_equity = Some(shareholders_equity);
  }

  pub fn with_shareholders_equity(mut self, shareholders_equity: i64) -> Financials {
    self.shareholders_equity = Some(shareholders_equity);
    self
  }

  pub fn shareholders_equity(&self) -> Option<&i64> {
    self.shareholders_equity.as_ref()
  }

  pub fn reset_shareholders_equity(&mut self) {
    self.shareholders_equity = None;
  }

  pub fn set_average_equity(&mut self, average_equity: i64) {
    self.average_equity = Some(average_equity);
  }

  pub fn with_average_equity(mut self, average_equity: i64) -> Financials {
    self.average_equity = Some(average_equity);
    self
  }

  pub fn average_equity(&self) -> Option<&i64> {
    self.average_equity.as_ref()
  }

  pub fn reset_average_equity(&mut self) {
    self.average_equity = None;
  }

  pub fn set_shareholders_equity_usd(&mut self, shareholders_equity_usd: i64) {
    self.shareholders_equity_usd = Some(shareholders_equity_usd);
  }

  pub fn with_shareholders_equity_usd(mut self, shareholders_equity_usd: i64) -> Financials {
    self.shareholders_equity_usd = Some(shareholders_equity_usd);
    self
  }

  pub fn shareholders_equity_usd(&self) -> Option<&i64> {
    self.shareholders_equity_usd.as_ref()
  }

  pub fn reset_shareholders_equity_usd(&mut self) {
    self.shareholders_equity_usd = None;
  }

  pub fn set_enterprise_value(&mut self, enterprise_value: i64) {
    self.enterprise_value = Some(enterprise_value);
  }

  pub fn with_enterprise_value(mut self, enterprise_value: i64) -> Financials {
    self.enterprise_value = Some(enterprise_value);
    self
  }

  pub fn enterprise_value(&self) -> Option<&i64> {
    self.enterprise_value.as_ref()
  }

  pub fn reset_enterprise_value(&mut self) {
    self.enterprise_value = None;
  }

  pub fn set_enterprise_value_over_ebit(&mut self, enterprise_value_over_ebit: i64) {
    self.enterprise_value_over_ebit = Some(enterprise_value_over_ebit);
  }

  pub fn with_enterprise_value_over_ebit(mut self, enterprise_value_over_ebit: i64) -> Financials {
    self.enterprise_value_over_ebit = Some(enterprise_value_over_ebit);
    self
  }

  pub fn enterprise_value_over_ebit(&self) -> Option<&i64> {
    self.enterprise_value_over_ebit.as_ref()
  }

  pub fn reset_enterprise_value_over_ebit(&mut self) {
    self.enterprise_value_over_ebit = None;
  }

  pub fn set_enterprise_value_over_ebitda(&mut self, enterprise_value_over_ebitda: i64) {
    self.enterprise_value_over_ebitda = Some(enterprise_value_over_ebitda);
  }

  pub fn with_enterprise_value_over_ebitda(mut self, enterprise_value_over_ebitda: i64) -> Financials {
    self.enterprise_value_over_ebitda = Some(enterprise_value_over_ebitda);
    self
  }

  pub fn enterprise_value_over_ebitda(&self) -> Option<&i64> {
    self.enterprise_value_over_ebitda.as_ref()
  }

  pub fn reset_enterprise_value_over_ebitda(&mut self) {
    self.enterprise_value_over_ebitda = None;
  }

  pub fn set_free_cash_flow(&mut self, free_cash_flow: i64) {
    self.free_cash_flow = Some(free_cash_flow);
  }

  pub fn with_free_cash_flow(mut self, free_cash_flow: i64) -> Financials {
    self.free_cash_flow = Some(free_cash_flow);
    self
  }

  pub fn free_cash_flow(&self) -> Option<&i64> {
    self.free_cash_flow.as_ref()
  }

  pub fn reset_free_cash_flow(&mut self) {
    self.free_cash_flow = None;
  }

  pub fn set_free_cash_flow_per_share(&mut self, free_cash_flow_per_share: i64) {
    self.free_cash_flow_per_share = Some(free_cash_flow_per_share);
  }

  pub fn with_free_cash_flow_per_share(mut self, free_cash_flow_per_share: i64) -> Financials {
    self.free_cash_flow_per_share = Some(free_cash_flow_per_share);
    self
  }

  pub fn free_cash_flow_per_share(&self) -> Option<&i64> {
    self.free_cash_flow_per_share.as_ref()
  }

  pub fn reset_free_cash_flow_per_share(&mut self) {
    self.free_cash_flow_per_share = None;
  }

  pub fn set_foreign_currency_usd_exchange_rate(&mut self, foreign_currency_usd_exchange_rate: i64) {
    self.foreign_currency_usd_exchange_rate = Some(foreign_currency_usd_exchange_rate);
  }

  pub fn with_foreign_currency_usd_exchange_rate(mut self, foreign_currency_usd_exchange_rate: i64) -> Financials {
    self.foreign_currency_usd_exchange_rate = Some(foreign_currency_usd_exchange_rate);
    self
  }

  pub fn foreign_currency_usd_exchange_rate(&self) -> Option<&i64> {
    self.foreign_currency_usd_exchange_rate.as_ref()
  }

  pub fn reset_foreign_currency_usd_exchange_rate(&mut self) {
    self.foreign_currency_usd_exchange_rate = None;
  }

  pub fn set_gross_profit(&mut self, gross_profit: i64) {
    self.gross_profit = Some(gross_profit);
  }

  pub fn with_gross_profit(mut self, gross_profit: i64) -> Financials {
    self.gross_profit = Some(gross_profit);
    self
  }

  pub fn gross_profit(&self) -> Option<&i64> {
    self.gross_profit.as_ref()
  }

  pub fn reset_gross_profit(&mut self) {
    self.gross_profit = None;
  }

  pub fn set_gross_margin(&mut self, gross_margin: i64) {
    self.gross_margin = Some(gross_margin);
  }

  pub fn with_gross_margin(mut self, gross_margin: i64) -> Financials {
    self.gross_margin = Some(gross_margin);
    self
  }

  pub fn gross_margin(&self) -> Option<&i64> {
    self.gross_margin.as_ref()
  }

  pub fn reset_gross_margin(&mut self) {
    self.gross_margin = None;
  }

  pub fn set_goodwill_and_intangible_assets(&mut self, goodwill_and_intangible_assets: i64) {
    self.goodwill_and_intangible_assets = Some(goodwill_and_intangible_assets);
  }

  pub fn with_goodwill_and_intangible_assets(mut self, goodwill_and_intangible_assets: i64) -> Financials {
    self.goodwill_and_intangible_assets = Some(goodwill_and_intangible_assets);
    self
  }

  pub fn goodwill_and_intangible_assets(&self) -> Option<&i64> {
    self.goodwill_and_intangible_assets.as_ref()
  }

  pub fn reset_goodwill_and_intangible_assets(&mut self) {
    self.goodwill_and_intangible_assets = None;
  }

  pub fn set_interest_expense(&mut self, interest_expense: i64) {
    self.interest_expense = Some(interest_expense);
  }

  pub fn with_interest_expense(mut self, interest_expense: i64) -> Financials {
    self.interest_expense = Some(interest_expense);
    self
  }

  pub fn interest_expense(&self) -> Option<&i64> {
    self.interest_expense.as_ref()
  }

  pub fn reset_interest_expense(&mut self) {
    self.interest_expense = None;
  }

  pub fn set_invested_capital(&mut self, invested_capital: i64) {
    self.invested_capital = Some(invested_capital);
  }

  pub fn with_invested_capital(mut self, invested_capital: i64) -> Financials {
    self.invested_capital = Some(invested_capital);
    self
  }

  pub fn invested_capital(&self) -> Option<&i64> {
    self.invested_capital.as_ref()
  }

  pub fn reset_invested_capital(&mut self) {
    self.invested_capital = None;
  }

  pub fn set_invested_capital_average(&mut self, invested_capital_average: i64) {
    self.invested_capital_average = Some(invested_capital_average);
  }

  pub fn with_invested_capital_average(mut self, invested_capital_average: i64) -> Financials {
    self.invested_capital_average = Some(invested_capital_average);
    self
  }

  pub fn invested_capital_average(&self) -> Option<&i64> {
    self.invested_capital_average.as_ref()
  }

  pub fn reset_invested_capital_average(&mut self) {
    self.invested_capital_average = None;
  }

  pub fn set_inventory(&mut self, inventory: i64) {
    self.inventory = Some(inventory);
  }

  pub fn with_inventory(mut self, inventory: i64) -> Financials {
    self.inventory = Some(inventory);
    self
  }

  pub fn inventory(&self) -> Option<&i64> {
    self.inventory.as_ref()
  }

  pub fn reset_inventory(&mut self) {
    self.inventory = None;
  }

  pub fn set_investments(&mut self, investments: i64) {
    self.investments = Some(investments);
  }

  pub fn with_investments(mut self, investments: i64) -> Financials {
    self.investments = Some(investments);
    self
  }

  pub fn investments(&self) -> Option<&i64> {
    self.investments.as_ref()
  }

  pub fn reset_investments(&mut self) {
    self.investments = None;
  }

  pub fn set_investments_current(&mut self, investments_current: i64) {
    self.investments_current = Some(investments_current);
  }

  pub fn with_investments_current(mut self, investments_current: i64) -> Financials {
    self.investments_current = Some(investments_current);
    self
  }

  pub fn investments_current(&self) -> Option<&i64> {
    self.investments_current.as_ref()
  }

  pub fn reset_investments_current(&mut self) {
    self.investments_current = None;
  }

  pub fn set_investments_non_current(&mut self, investments_non_current: i64) {
    self.investments_non_current = Some(investments_non_current);
  }

  pub fn with_investments_non_current(mut self, investments_non_current: i64) -> Financials {
    self.investments_non_current = Some(investments_non_current);
    self
  }

  pub fn investments_non_current(&self) -> Option<&i64> {
    self.investments_non_current.as_ref()
  }

  pub fn reset_investments_non_current(&mut self) {
    self.investments_non_current = None;
  }

  pub fn set_total_liabilities(&mut self, total_liabilities: i64) {
    self.total_liabilities = Some(total_liabilities);
  }

  pub fn with_total_liabilities(mut self, total_liabilities: i64) -> Financials {
    self.total_liabilities = Some(total_liabilities);
    self
  }

  pub fn total_liabilities(&self) -> Option<&i64> {
    self.total_liabilities.as_ref()
  }

  pub fn reset_total_liabilities(&mut self) {
    self.total_liabilities = None;
  }

  pub fn set_current_liabilities(&mut self, current_liabilities: i64) {
    self.current_liabilities = Some(current_liabilities);
  }

  pub fn with_current_liabilities(mut self, current_liabilities: i64) -> Financials {
    self.current_liabilities = Some(current_liabilities);
    self
  }

  pub fn current_liabilities(&self) -> Option<&i64> {
    self.current_liabilities.as_ref()
  }

  pub fn reset_current_liabilities(&mut self) {
    self.current_liabilities = None;
  }

  pub fn set_liabilities_non_current(&mut self, liabilities_non_current: i64) {
    self.liabilities_non_current = Some(liabilities_non_current);
  }

  pub fn with_liabilities_non_current(mut self, liabilities_non_current: i64) -> Financials {
    self.liabilities_non_current = Some(liabilities_non_current);
    self
  }

  pub fn liabilities_non_current(&self) -> Option<&i64> {
    self.liabilities_non_current.as_ref()
  }

  pub fn reset_liabilities_non_current(&mut self) {
    self.liabilities_non_current = None;
  }

  pub fn set_market_capitalization(&mut self, market_capitalization: i64) {
    self.market_capitalization = Some(market_capitalization);
  }

  pub fn with_market_capitalization(mut self, market_capitalization: i64) -> Financials {
    self.market_capitalization = Some(market_capitalization);
    self
  }

  pub fn market_capitalization(&self) -> Option<&i64> {
    self.market_capitalization.as_ref()
  }

  pub fn reset_market_capitalization(&mut self) {
    self.market_capitalization = None;
  }

  pub fn set_net_cash_flow(&mut self, net_cash_flow: i64) {
    self.net_cash_flow = Some(net_cash_flow);
  }

  pub fn with_net_cash_flow(mut self, net_cash_flow: i64) -> Financials {
    self.net_cash_flow = Some(net_cash_flow);
    self
  }

  pub fn net_cash_flow(&self) -> Option<&i64> {
    self.net_cash_flow.as_ref()
  }

  pub fn reset_net_cash_flow(&mut self) {
    self.net_cash_flow = None;
  }

  pub fn set_net_cash_flow_business_acquisitions_disposals(&mut self, net_cash_flow_business_acquisitions_disposals: i64) {
    self.net_cash_flow_business_acquisitions_disposals = Some(net_cash_flow_business_acquisitions_disposals);
  }

  pub fn with_net_cash_flow_business_acquisitions_disposals(mut self, net_cash_flow_business_acquisitions_disposals: i64) -> Financials {
    self.net_cash_flow_business_acquisitions_disposals = Some(net_cash_flow_business_acquisitions_disposals);
    self
  }

  pub fn net_cash_flow_business_acquisitions_disposals(&self) -> Option<&i64> {
    self.net_cash_flow_business_acquisitions_disposals.as_ref()
  }

  pub fn reset_net_cash_flow_business_acquisitions_disposals(&mut self) {
    self.net_cash_flow_business_acquisitions_disposals = None;
  }

  pub fn set_issuance_equity_shares(&mut self, issuance_equity_shares: i64) {
    self.issuance_equity_shares = Some(issuance_equity_shares);
  }

  pub fn with_issuance_equity_shares(mut self, issuance_equity_shares: i64) -> Financials {
    self.issuance_equity_shares = Some(issuance_equity_shares);
    self
  }

  pub fn issuance_equity_shares(&self) -> Option<&i64> {
    self.issuance_equity_shares.as_ref()
  }

  pub fn reset_issuance_equity_shares(&mut self) {
    self.issuance_equity_shares = None;
  }

  pub fn set_issuance_debt_securities(&mut self, issuance_debt_securities: i64) {
    self.issuance_debt_securities = Some(issuance_debt_securities);
  }

  pub fn with_issuance_debt_securities(mut self, issuance_debt_securities: i64) -> Financials {
    self.issuance_debt_securities = Some(issuance_debt_securities);
    self
  }

  pub fn issuance_debt_securities(&self) -> Option<&i64> {
    self.issuance_debt_securities.as_ref()
  }

  pub fn reset_issuance_debt_securities(&mut self) {
    self.issuance_debt_securities = None;
  }

  pub fn set_payment_dividends_other_cash_distributions(&mut self, payment_dividends_other_cash_distributions: i64) {
    self.payment_dividends_other_cash_distributions = Some(payment_dividends_other_cash_distributions);
  }

  pub fn with_payment_dividends_other_cash_distributions(mut self, payment_dividends_other_cash_distributions: i64) -> Financials {
    self.payment_dividends_other_cash_distributions = Some(payment_dividends_other_cash_distributions);
    self
  }

  pub fn payment_dividends_other_cash_distributions(&self) -> Option<&i64> {
    self.payment_dividends_other_cash_distributions.as_ref()
  }

  pub fn reset_payment_dividends_other_cash_distributions(&mut self) {
    self.payment_dividends_other_cash_distributions = None;
  }

  pub fn set_net_cash_flow_from_financing(&mut self, net_cash_flow_from_financing: i64) {
    self.net_cash_flow_from_financing = Some(net_cash_flow_from_financing);
  }

  pub fn with_net_cash_flow_from_financing(mut self, net_cash_flow_from_financing: i64) -> Financials {
    self.net_cash_flow_from_financing = Some(net_cash_flow_from_financing);
    self
  }

  pub fn net_cash_flow_from_financing(&self) -> Option<&i64> {
    self.net_cash_flow_from_financing.as_ref()
  }

  pub fn reset_net_cash_flow_from_financing(&mut self) {
    self.net_cash_flow_from_financing = None;
  }

  pub fn set_net_cash_flow_from_investing(&mut self, net_cash_flow_from_investing: i64) {
    self.net_cash_flow_from_investing = Some(net_cash_flow_from_investing);
  }

  pub fn with_net_cash_flow_from_investing(mut self, net_cash_flow_from_investing: i64) -> Financials {
    self.net_cash_flow_from_investing = Some(net_cash_flow_from_investing);
    self
  }

  pub fn net_cash_flow_from_investing(&self) -> Option<&i64> {
    self.net_cash_flow_from_investing.as_ref()
  }

  pub fn reset_net_cash_flow_from_investing(&mut self) {
    self.net_cash_flow_from_investing = None;
  }

  pub fn set_net_cash_flow_investment_acquisitions_disposals(&mut self, net_cash_flow_investment_acquisitions_disposals: i64) {
    self.net_cash_flow_investment_acquisitions_disposals = Some(net_cash_flow_investment_acquisitions_disposals);
  }

  pub fn with_net_cash_flow_investment_acquisitions_disposals(mut self, net_cash_flow_investment_acquisitions_disposals: i64) -> Financials {
    self.net_cash_flow_investment_acquisitions_disposals = Some(net_cash_flow_investment_acquisitions_disposals);
    self
  }

  pub fn net_cash_flow_investment_acquisitions_disposals(&self) -> Option<&i64> {
    self.net_cash_flow_investment_acquisitions_disposals.as_ref()
  }

  pub fn reset_net_cash_flow_investment_acquisitions_disposals(&mut self) {
    self.net_cash_flow_investment_acquisitions_disposals = None;
  }

  pub fn set_net_cash_flow_from_operations(&mut self, net_cash_flow_from_operations: i64) {
    self.net_cash_flow_from_operations = Some(net_cash_flow_from_operations);
  }

  pub fn with_net_cash_flow_from_operations(mut self, net_cash_flow_from_operations: i64) -> Financials {
    self.net_cash_flow_from_operations = Some(net_cash_flow_from_operations);
    self
  }

  pub fn net_cash_flow_from_operations(&self) -> Option<&i64> {
    self.net_cash_flow_from_operations.as_ref()
  }

  pub fn reset_net_cash_flow_from_operations(&mut self) {
    self.net_cash_flow_from_operations = None;
  }

  pub fn set_effect_of_exchange_rate_changes_on_cash(&mut self, effect_of_exchange_rate_changes_on_cash: i64) {
    self.effect_of_exchange_rate_changes_on_cash = Some(effect_of_exchange_rate_changes_on_cash);
  }

  pub fn with_effect_of_exchange_rate_changes_on_cash(mut self, effect_of_exchange_rate_changes_on_cash: i64) -> Financials {
    self.effect_of_exchange_rate_changes_on_cash = Some(effect_of_exchange_rate_changes_on_cash);
    self
  }

  pub fn effect_of_exchange_rate_changes_on_cash(&self) -> Option<&i64> {
    self.effect_of_exchange_rate_changes_on_cash.as_ref()
  }

  pub fn reset_effect_of_exchange_rate_changes_on_cash(&mut self) {
    self.effect_of_exchange_rate_changes_on_cash = None;
  }

  pub fn set_net_income(&mut self, net_income: i64) {
    self.net_income = Some(net_income);
  }

  pub fn with_net_income(mut self, net_income: i64) -> Financials {
    self.net_income = Some(net_income);
    self
  }

  pub fn net_income(&self) -> Option<&i64> {
    self.net_income.as_ref()
  }

  pub fn reset_net_income(&mut self) {
    self.net_income = None;
  }

  pub fn set_net_income_common_stock(&mut self, net_income_common_stock: i64) {
    self.net_income_common_stock = Some(net_income_common_stock);
  }

  pub fn with_net_income_common_stock(mut self, net_income_common_stock: i64) -> Financials {
    self.net_income_common_stock = Some(net_income_common_stock);
    self
  }

  pub fn net_income_common_stock(&self) -> Option<&i64> {
    self.net_income_common_stock.as_ref()
  }

  pub fn reset_net_income_common_stock(&mut self) {
    self.net_income_common_stock = None;
  }

  pub fn set_net_income_common_stock_usd(&mut self, net_income_common_stock_usd: i64) {
    self.net_income_common_stock_usd = Some(net_income_common_stock_usd);
  }

  pub fn with_net_income_common_stock_usd(mut self, net_income_common_stock_usd: i64) -> Financials {
    self.net_income_common_stock_usd = Some(net_income_common_stock_usd);
    self
  }

  pub fn net_income_common_stock_usd(&self) -> Option<&i64> {
    self.net_income_common_stock_usd.as_ref()
  }

  pub fn reset_net_income_common_stock_usd(&mut self) {
    self.net_income_common_stock_usd = None;
  }

  pub fn set_net_loss_income_from_discontinued_operations(&mut self, net_loss_income_from_discontinued_operations: i64) {
    self.net_loss_income_from_discontinued_operations = Some(net_loss_income_from_discontinued_operations);
  }

  pub fn with_net_loss_income_from_discontinued_operations(mut self, net_loss_income_from_discontinued_operations: i64) -> Financials {
    self.net_loss_income_from_discontinued_operations = Some(net_loss_income_from_discontinued_operations);
    self
  }

  pub fn net_loss_income_from_discontinued_operations(&self) -> Option<&i64> {
    self.net_loss_income_from_discontinued_operations.as_ref()
  }

  pub fn reset_net_loss_income_from_discontinued_operations(&mut self) {
    self.net_loss_income_from_discontinued_operations = None;
  }

  pub fn set_net_income_to_non_controlling_interests(&mut self, net_income_to_non_controlling_interests: i64) {
    self.net_income_to_non_controlling_interests = Some(net_income_to_non_controlling_interests);
  }

  pub fn with_net_income_to_non_controlling_interests(mut self, net_income_to_non_controlling_interests: i64) -> Financials {
    self.net_income_to_non_controlling_interests = Some(net_income_to_non_controlling_interests);
    self
  }

  pub fn net_income_to_non_controlling_interests(&self) -> Option<&i64> {
    self.net_income_to_non_controlling_interests.as_ref()
  }

  pub fn reset_net_income_to_non_controlling_interests(&mut self) {
    self.net_income_to_non_controlling_interests = None;
  }

  pub fn set_profit_margin(&mut self, profit_margin: i64) {
    self.profit_margin = Some(profit_margin);
  }

  pub fn with_profit_margin(mut self, profit_margin: i64) -> Financials {
    self.profit_margin = Some(profit_margin);
    self
  }

  pub fn profit_margin(&self) -> Option<&i64> {
    self.profit_margin.as_ref()
  }

  pub fn reset_profit_margin(&mut self) {
    self.profit_margin = None;
  }

  pub fn set_operating_expenses(&mut self, operating_expenses: i64) {
    self.operating_expenses = Some(operating_expenses);
  }

  pub fn with_operating_expenses(mut self, operating_expenses: i64) -> Financials {
    self.operating_expenses = Some(operating_expenses);
    self
  }

  pub fn operating_expenses(&self) -> Option<&i64> {
    self.operating_expenses.as_ref()
  }

  pub fn reset_operating_expenses(&mut self) {
    self.operating_expenses = None;
  }

  pub fn set_operating_income(&mut self, operating_income: i64) {
    self.operating_income = Some(operating_income);
  }

  pub fn with_operating_income(mut self, operating_income: i64) -> Financials {
    self.operating_income = Some(operating_income);
    self
  }

  pub fn operating_income(&self) -> Option<&i64> {
    self.operating_income.as_ref()
  }

  pub fn reset_operating_income(&mut self) {
    self.operating_income = None;
  }

  pub fn set_trade_and_non_trade_payables(&mut self, trade_and_non_trade_payables: i64) {
    self.trade_and_non_trade_payables = Some(trade_and_non_trade_payables);
  }

  pub fn with_trade_and_non_trade_payables(mut self, trade_and_non_trade_payables: i64) -> Financials {
    self.trade_and_non_trade_payables = Some(trade_and_non_trade_payables);
    self
  }

  pub fn trade_and_non_trade_payables(&self) -> Option<&i64> {
    self.trade_and_non_trade_payables.as_ref()
  }

  pub fn reset_trade_and_non_trade_payables(&mut self) {
    self.trade_and_non_trade_payables = None;
  }

  pub fn set_payout_ratio(&mut self, payout_ratio: i64) {
    self.payout_ratio = Some(payout_ratio);
  }

  pub fn with_payout_ratio(mut self, payout_ratio: i64) -> Financials {
    self.payout_ratio = Some(payout_ratio);
    self
  }

  pub fn payout_ratio(&self) -> Option<&i64> {
    self.payout_ratio.as_ref()
  }

  pub fn reset_payout_ratio(&mut self) {
    self.payout_ratio = None;
  }

  pub fn set_price_to_book_value(&mut self, price_to_book_value: i64) {
    self.price_to_book_value = Some(price_to_book_value);
  }

  pub fn with_price_to_book_value(mut self, price_to_book_value: i64) -> Financials {
    self.price_to_book_value = Some(price_to_book_value);
    self
  }

  pub fn price_to_book_value(&self) -> Option<&i64> {
    self.price_to_book_value.as_ref()
  }

  pub fn reset_price_to_book_value(&mut self) {
    self.price_to_book_value = None;
  }

  pub fn set_price_earnings(&mut self, price_earnings: i64) {
    self.price_earnings = Some(price_earnings);
  }

  pub fn with_price_earnings(mut self, price_earnings: i64) -> Financials {
    self.price_earnings = Some(price_earnings);
    self
  }

  pub fn price_earnings(&self) -> Option<&i64> {
    self.price_earnings.as_ref()
  }

  pub fn reset_price_earnings(&mut self) {
    self.price_earnings = None;
  }

  pub fn set_price_to_earnings_ratio(&mut self, price_to_earnings_ratio: i64) {
    self.price_to_earnings_ratio = Some(price_to_earnings_ratio);
  }

  pub fn with_price_to_earnings_ratio(mut self, price_to_earnings_ratio: i64) -> Financials {
    self.price_to_earnings_ratio = Some(price_to_earnings_ratio);
    self
  }

  pub fn price_to_earnings_ratio(&self) -> Option<&i64> {
    self.price_to_earnings_ratio.as_ref()
  }

  pub fn reset_price_to_earnings_ratio(&mut self) {
    self.price_to_earnings_ratio = None;
  }

  pub fn set_property_plant_equipment_net(&mut self, property_plant_equipment_net: i64) {
    self.property_plant_equipment_net = Some(property_plant_equipment_net);
  }

  pub fn with_property_plant_equipment_net(mut self, property_plant_equipment_net: i64) -> Financials {
    self.property_plant_equipment_net = Some(property_plant_equipment_net);
    self
  }

  pub fn property_plant_equipment_net(&self) -> Option<&i64> {
    self.property_plant_equipment_net.as_ref()
  }

  pub fn reset_property_plant_equipment_net(&mut self) {
    self.property_plant_equipment_net = None;
  }

  pub fn set_preferred_dividends_income_statement_impact(&mut self, preferred_dividends_income_statement_impact: i64) {
    self.preferred_dividends_income_statement_impact = Some(preferred_dividends_income_statement_impact);
  }

  pub fn with_preferred_dividends_income_statement_impact(mut self, preferred_dividends_income_statement_impact: i64) -> Financials {
    self.preferred_dividends_income_statement_impact = Some(preferred_dividends_income_statement_impact);
    self
  }

  pub fn preferred_dividends_income_statement_impact(&self) -> Option<&i64> {
    self.preferred_dividends_income_statement_impact.as_ref()
  }

  pub fn reset_preferred_dividends_income_statement_impact(&mut self) {
    self.preferred_dividends_income_statement_impact = None;
  }

  pub fn set_share_price_adjusted_close(&mut self, share_price_adjusted_close: i64) {
    self.share_price_adjusted_close = Some(share_price_adjusted_close);
  }

  pub fn with_share_price_adjusted_close(mut self, share_price_adjusted_close: i64) -> Financials {
    self.share_price_adjusted_close = Some(share_price_adjusted_close);
    self
  }

  pub fn share_price_adjusted_close(&self) -> Option<&i64> {
    self.share_price_adjusted_close.as_ref()
  }

  pub fn reset_share_price_adjusted_close(&mut self) {
    self.share_price_adjusted_close = None;
  }

  pub fn set_price_sales(&mut self, price_sales: i64) {
    self.price_sales = Some(price_sales);
  }

  pub fn with_price_sales(mut self, price_sales: i64) -> Financials {
    self.price_sales = Some(price_sales);
    self
  }

  pub fn price_sales(&self) -> Option<&i64> {
    self.price_sales.as_ref()
  }

  pub fn reset_price_sales(&mut self) {
    self.price_sales = None;
  }

  pub fn set_price_to_sales_ratio(&mut self, price_to_sales_ratio: i64) {
    self.price_to_sales_ratio = Some(price_to_sales_ratio);
  }

  pub fn with_price_to_sales_ratio(mut self, price_to_sales_ratio: i64) -> Financials {
    self.price_to_sales_ratio = Some(price_to_sales_ratio);
    self
  }

  pub fn price_to_sales_ratio(&self) -> Option<&i64> {
    self.price_to_sales_ratio.as_ref()
  }

  pub fn reset_price_to_sales_ratio(&mut self) {
    self.price_to_sales_ratio = None;
  }

  pub fn set_trade_and_non_trade_receivables(&mut self, trade_and_non_trade_receivables: i64) {
    self.trade_and_non_trade_receivables = Some(trade_and_non_trade_receivables);
  }

  pub fn with_trade_and_non_trade_receivables(mut self, trade_and_non_trade_receivables: i64) -> Financials {
    self.trade_and_non_trade_receivables = Some(trade_and_non_trade_receivables);
    self
  }

  pub fn trade_and_non_trade_receivables(&self) -> Option<&i64> {
    self.trade_and_non_trade_receivables.as_ref()
  }

  pub fn reset_trade_and_non_trade_receivables(&mut self) {
    self.trade_and_non_trade_receivables = None;
  }

  pub fn set_accumulated_retained_earnings_deficit(&mut self, accumulated_retained_earnings_deficit: i64) {
    self.accumulated_retained_earnings_deficit = Some(accumulated_retained_earnings_deficit);
  }

  pub fn with_accumulated_retained_earnings_deficit(mut self, accumulated_retained_earnings_deficit: i64) -> Financials {
    self.accumulated_retained_earnings_deficit = Some(accumulated_retained_earnings_deficit);
    self
  }

  pub fn accumulated_retained_earnings_deficit(&self) -> Option<&i64> {
    self.accumulated_retained_earnings_deficit.as_ref()
  }

  pub fn reset_accumulated_retained_earnings_deficit(&mut self) {
    self.accumulated_retained_earnings_deficit = None;
  }

  pub fn set_revenues(&mut self, revenues: i64) {
    self.revenues = Some(revenues);
  }

  pub fn with_revenues(mut self, revenues: i64) -> Financials {
    self.revenues = Some(revenues);
    self
  }

  pub fn revenues(&self) -> Option<&i64> {
    self.revenues.as_ref()
  }

  pub fn reset_revenues(&mut self) {
    self.revenues = None;
  }

  pub fn set_revenues_usd(&mut self, revenues_usd: i64) {
    self.revenues_usd = Some(revenues_usd);
  }

  pub fn with_revenues_usd(mut self, revenues_usd: i64) -> Financials {
    self.revenues_usd = Some(revenues_usd);
    self
  }

  pub fn revenues_usd(&self) -> Option<&i64> {
    self.revenues_usd.as_ref()
  }

  pub fn reset_revenues_usd(&mut self) {
    self.revenues_usd = None;
  }

  pub fn set_research_and_development_expense(&mut self, research_and_development_expense: i64) {
    self.research_and_development_expense = Some(research_and_development_expense);
  }

  pub fn with_research_and_development_expense(mut self, research_and_development_expense: i64) -> Financials {
    self.research_and_development_expense = Some(research_and_development_expense);
    self
  }

  pub fn research_and_development_expense(&self) -> Option<&i64> {
    self.research_and_development_expense.as_ref()
  }

  pub fn reset_research_and_development_expense(&mut self) {
    self.research_and_development_expense = None;
  }

  pub fn set_return_on_average_assets(&mut self, return_on_average_assets: i64) {
    self.return_on_average_assets = Some(return_on_average_assets);
  }

  pub fn with_return_on_average_assets(mut self, return_on_average_assets: i64) -> Financials {
    self.return_on_average_assets = Some(return_on_average_assets);
    self
  }

  pub fn return_on_average_assets(&self) -> Option<&i64> {
    self.return_on_average_assets.as_ref()
  }

  pub fn reset_return_on_average_assets(&mut self) {
    self.return_on_average_assets = None;
  }

  pub fn set_return_on_average_equity(&mut self, return_on_average_equity: i64) {
    self.return_on_average_equity = Some(return_on_average_equity);
  }

  pub fn with_return_on_average_equity(mut self, return_on_average_equity: i64) -> Financials {
    self.return_on_average_equity = Some(return_on_average_equity);
    self
  }

  pub fn return_on_average_equity(&self) -> Option<&i64> {
    self.return_on_average_equity.as_ref()
  }

  pub fn reset_return_on_average_equity(&mut self) {
    self.return_on_average_equity = None;
  }

  pub fn set_return_on_invested_capital(&mut self, return_on_invested_capital: i64) {
    self.return_on_invested_capital = Some(return_on_invested_capital);
  }

  pub fn with_return_on_invested_capital(mut self, return_on_invested_capital: i64) -> Financials {
    self.return_on_invested_capital = Some(return_on_invested_capital);
    self
  }

  pub fn return_on_invested_capital(&self) -> Option<&i64> {
    self.return_on_invested_capital.as_ref()
  }

  pub fn reset_return_on_invested_capital(&mut self) {
    self.return_on_invested_capital = None;
  }

  pub fn set_return_on_sales(&mut self, return_on_sales: i64) {
    self.return_on_sales = Some(return_on_sales);
  }

  pub fn with_return_on_sales(mut self, return_on_sales: i64) -> Financials {
    self.return_on_sales = Some(return_on_sales);
    self
  }

  pub fn return_on_sales(&self) -> Option<&i64> {
    self.return_on_sales.as_ref()
  }

  pub fn reset_return_on_sales(&mut self) {
    self.return_on_sales = None;
  }

  pub fn set_share_based_compensation(&mut self, share_based_compensation: i64) {
    self.share_based_compensation = Some(share_based_compensation);
  }

  pub fn with_share_based_compensation(mut self, share_based_compensation: i64) -> Financials {
    self.share_based_compensation = Some(share_based_compensation);
    self
  }

  pub fn share_based_compensation(&self) -> Option<&i64> {
    self.share_based_compensation.as_ref()
  }

  pub fn reset_share_based_compensation(&mut self) {
    self.share_based_compensation = None;
  }

  pub fn set_selling_general_and_administrative_expense(&mut self, selling_general_and_administrative_expense: i64) {
    self.selling_general_and_administrative_expense = Some(selling_general_and_administrative_expense);
  }

  pub fn with_selling_general_and_administrative_expense(mut self, selling_general_and_administrative_expense: i64) -> Financials {
    self.selling_general_and_administrative_expense = Some(selling_general_and_administrative_expense);
    self
  }

  pub fn selling_general_and_administrative_expense(&self) -> Option<&i64> {
    self.selling_general_and_administrative_expense.as_ref()
  }

  pub fn reset_selling_general_and_administrative_expense(&mut self) {
    self.selling_general_and_administrative_expense = None;
  }

  pub fn set_share_factor(&mut self, share_factor: i64) {
    self.share_factor = Some(share_factor);
  }

  pub fn with_share_factor(mut self, share_factor: i64) -> Financials {
    self.share_factor = Some(share_factor);
    self
  }

  pub fn share_factor(&self) -> Option<&i64> {
    self.share_factor.as_ref()
  }

  pub fn reset_share_factor(&mut self) {
    self.share_factor = None;
  }

  pub fn set_shares(&mut self, shares: i64) {
    self.shares = Some(shares);
  }

  pub fn with_shares(mut self, shares: i64) -> Financials {
    self.shares = Some(shares);
    self
  }

  pub fn shares(&self) -> Option<&i64> {
    self.shares.as_ref()
  }

  pub fn reset_shares(&mut self) {
    self.shares = None;
  }

  pub fn set_weighted_average_shares(&mut self, weighted_average_shares: i64) {
    self.weighted_average_shares = Some(weighted_average_shares);
  }

  pub fn with_weighted_average_shares(mut self, weighted_average_shares: i64) -> Financials {
    self.weighted_average_shares = Some(weighted_average_shares);
    self
  }

  pub fn weighted_average_shares(&self) -> Option<&i64> {
    self.weighted_average_shares.as_ref()
  }

  pub fn reset_weighted_average_shares(&mut self) {
    self.weighted_average_shares = None;
  }

  pub fn set_weighted_average_shares_diluted(&mut self, weighted_average_shares_diluted: i64) {
    self.weighted_average_shares_diluted = Some(weighted_average_shares_diluted);
  }

  pub fn with_weighted_average_shares_diluted(mut self, weighted_average_shares_diluted: i64) -> Financials {
    self.weighted_average_shares_diluted = Some(weighted_average_shares_diluted);
    self
  }

  pub fn weighted_average_shares_diluted(&self) -> Option<&i64> {
    self.weighted_average_shares_diluted.as_ref()
  }

  pub fn reset_weighted_average_shares_diluted(&mut self) {
    self.weighted_average_shares_diluted = None;
  }

  pub fn set_sales_per_share(&mut self, sales_per_share: i64) {
    self.sales_per_share = Some(sales_per_share);
  }

  pub fn with_sales_per_share(mut self, sales_per_share: i64) -> Financials {
    self.sales_per_share = Some(sales_per_share);
    self
  }

  pub fn sales_per_share(&self) -> Option<&i64> {
    self.sales_per_share.as_ref()
  }

  pub fn reset_sales_per_share(&mut self) {
    self.sales_per_share = None;
  }

  pub fn set_tangible_asset_value(&mut self, tangible_asset_value: i64) {
    self.tangible_asset_value = Some(tangible_asset_value);
  }

  pub fn with_tangible_asset_value(mut self, tangible_asset_value: i64) -> Financials {
    self.tangible_asset_value = Some(tangible_asset_value);
    self
  }

  pub fn tangible_asset_value(&self) -> Option<&i64> {
    self.tangible_asset_value.as_ref()
  }

  pub fn reset_tangible_asset_value(&mut self) {
    self.tangible_asset_value = None;
  }

  pub fn set_tax_assets(&mut self, tax_assets: i64) {
    self.tax_assets = Some(tax_assets);
  }

  pub fn with_tax_assets(mut self, tax_assets: i64) -> Financials {
    self.tax_assets = Some(tax_assets);
    self
  }

  pub fn tax_assets(&self) -> Option<&i64> {
    self.tax_assets.as_ref()
  }

  pub fn reset_tax_assets(&mut self) {
    self.tax_assets = None;
  }

  pub fn set_income_tax_expense(&mut self, income_tax_expense: i64) {
    self.income_tax_expense = Some(income_tax_expense);
  }

  pub fn with_income_tax_expense(mut self, income_tax_expense: i64) -> Financials {
    self.income_tax_expense = Some(income_tax_expense);
    self
  }

  pub fn income_tax_expense(&self) -> Option<&i64> {
    self.income_tax_expense.as_ref()
  }

  pub fn reset_income_tax_expense(&mut self) {
    self.income_tax_expense = None;
  }

  pub fn set_tax_liabilities(&mut self, tax_liabilities: i64) {
    self.tax_liabilities = Some(tax_liabilities);
  }

  pub fn with_tax_liabilities(mut self, tax_liabilities: i64) -> Financials {
    self.tax_liabilities = Some(tax_liabilities);
    self
  }

  pub fn tax_liabilities(&self) -> Option<&i64> {
    self.tax_liabilities.as_ref()
  }

  pub fn reset_tax_liabilities(&mut self) {
    self.tax_liabilities = None;
  }

  pub fn set_tangible_assets_book_value_per_share(&mut self, tangible_assets_book_value_per_share: i64) {
    self.tangible_assets_book_value_per_share = Some(tangible_assets_book_value_per_share);
  }

  pub fn with_tangible_assets_book_value_per_share(mut self, tangible_assets_book_value_per_share: i64) -> Financials {
    self.tangible_assets_book_value_per_share = Some(tangible_assets_book_value_per_share);
    self
  }

  pub fn tangible_assets_book_value_per_share(&self) -> Option<&i64> {
    self.tangible_assets_book_value_per_share.as_ref()
  }

  pub fn reset_tangible_assets_book_value_per_share(&mut self) {
    self.tangible_assets_book_value_per_share = None;
  }

  pub fn set_working_capital(&mut self, working_capital: i64) {
    self.working_capital = Some(working_capital);
  }

  pub fn with_working_capital(mut self, working_capital: i64) -> Financials {
    self.working_capital = Some(working_capital);
    self
  }

  pub fn working_capital(&self) -> Option<&i64> {
    self.working_capital.as_ref()
  }

  pub fn reset_working_capital(&mut self) {
    self.working_capital = None;
  }

}


