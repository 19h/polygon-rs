# Rust API client for pt

The future of fintech.

## Overview
This API client was generated by the [swagger-codegen](https://github.com/swagger-api/swagger-codegen) project.  By using the [swagger-spec](https://github.com/swagger-api/swagger-spec) from a remote server, you can easily generate an API client.

- API version: 1.0.1
- Package version: 0.2.4
- Build package: com.rust.codegen.RustGenerator

## Installation
Put the package under your project folder and add the following in import:
```
    "./pt"
```

## Documentation for API Endpoints

All URIs are relative to *https://api.polygon.io/*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*CryptoApi* | [**v1_historic_crypto_from_to_date_get**](CryptoApi.md#v1_historic_crypto_from_to_date_get) | **GET** v1/historic/crypto/{from}/{to}/{date} | Historic Crypto Trades
*CryptoApi* | [**v1_last_crypto_from_to_get**](CryptoApi.md#v1_last_crypto_from_to_get) | **GET** v1/last/crypto/{from}/{to} | Last Trade for a Crypto Pair
*CryptoApi* | [**v1_meta_crypto_exchanges_get**](CryptoApi.md#v1_meta_crypto_exchanges_get) | **GET** v1/meta/crypto-exchanges | Crypto Exchanges
*CryptoApi* | [**v1_open_close_crypto_from_to_date_get**](CryptoApi.md#v1_open_close_crypto_from_to_date_get) | **GET** v1/open-close/crypto/{from}/{to}/{date} | Daily Open / Close
*CryptoApi* | [**v2_aggs_grouped_locale_locale_market_market_date_get**](CryptoApi.md#v2_aggs_grouped_locale_locale_market_market_date_get) | **GET** v2/aggs/grouped/locale/{locale}/market/{market}/{date} | Grouped Daily ( Bars )
*CryptoApi* | [**v2_aggs_ticker_ticker_prev_get**](CryptoApi.md#v2_aggs_ticker_ticker_prev_get) | **GET** v2/aggs/ticker/{ticker}/prev | Previous Close
*CryptoApi* | [**v2_aggs_ticker_ticker_range_multiplier_timespan_from_to_get**](CryptoApi.md#v2_aggs_ticker_ticker_range_multiplier_timespan_from_to_get) | **GET** v2/aggs/ticker/{ticker}/range/{multiplier}/{timespan}/{from}/{to} | Aggregates ( Bars )
*CryptoApi* | [**v2_snapshot_locale_global_markets_crypto_direction_get**](CryptoApi.md#v2_snapshot_locale_global_markets_crypto_direction_get) | **GET** v2/snapshot/locale/global/markets/crypto/{direction} | Snapshot - Gainers / Losers
*CryptoApi* | [**v2_snapshot_locale_global_markets_crypto_tickers_get**](CryptoApi.md#v2_snapshot_locale_global_markets_crypto_tickers_get) | **GET** v2/snapshot/locale/global/markets/crypto/tickers | Snapshot - All Tickers
*CryptoApi* | [**v2_snapshot_locale_global_markets_crypto_tickers_ticker_book_get**](CryptoApi.md#v2_snapshot_locale_global_markets_crypto_tickers_ticker_book_get) | **GET** v2/snapshot/locale/global/markets/crypto/tickers/{ticker}/book | Snapshot - Single Ticker Full Book ( L2 )
*CryptoApi* | [**v2_snapshot_locale_global_markets_crypto_tickers_ticker_get**](CryptoApi.md#v2_snapshot_locale_global_markets_crypto_tickers_ticker_get) | **GET** v2/snapshot/locale/global/markets/crypto/tickers/{ticker} | Snapshot - Single Ticker
*ForexCurrenciesApi* | [**v1_conversion_from_to_get**](ForexCurrenciesApi.md#v1_conversion_from_to_get) | **GET** v1/conversion/{from}/{to} | Real-time Currency Conversion
*ForexCurrenciesApi* | [**v1_historic_forex_from_to_date_get**](ForexCurrenciesApi.md#v1_historic_forex_from_to_date_get) | **GET** v1/historic/forex/{from}/{to}/{date} | Historic Forex Ticks
*ForexCurrenciesApi* | [**v1_last_quote_currencies_from_to_get**](ForexCurrenciesApi.md#v1_last_quote_currencies_from_to_get) | **GET** v1/last_quote/currencies/{from}/{to} | Last Quote for a Currency Pair
*ForexCurrenciesApi* | [**v2_aggs_grouped_locale_locale_market_market_date_get**](ForexCurrenciesApi.md#v2_aggs_grouped_locale_locale_market_market_date_get) | **GET** v2/aggs/grouped/locale/{locale}/market/{market}/{date} | Grouped Daily ( Bars )
*ForexCurrenciesApi* | [**v2_aggs_ticker_ticker_prev_get**](ForexCurrenciesApi.md#v2_aggs_ticker_ticker_prev_get) | **GET** v2/aggs/ticker/{ticker}/prev | Previous Close
*ForexCurrenciesApi* | [**v2_aggs_ticker_ticker_range_multiplier_timespan_from_to_get**](ForexCurrenciesApi.md#v2_aggs_ticker_ticker_range_multiplier_timespan_from_to_get) | **GET** v2/aggs/ticker/{ticker}/range/{multiplier}/{timespan}/{from}/{to} | Aggregates ( Bars )
*ForexCurrenciesApi* | [**v2_snapshot_locale_global_markets_forex_direction_get**](ForexCurrenciesApi.md#v2_snapshot_locale_global_markets_forex_direction_get) | **GET** v2/snapshot/locale/global/markets/forex/{direction} | Snapshot - Gainers / Losers
*ForexCurrenciesApi* | [**v2_snapshot_locale_global_markets_forex_tickers_get**](ForexCurrenciesApi.md#v2_snapshot_locale_global_markets_forex_tickers_get) | **GET** v2/snapshot/locale/global/markets/forex/tickers | Snapshot - All Tickers
*ReferenceApi* | [**v1_marketstatus_now_get**](ReferenceApi.md#v1_marketstatus_now_get) | **GET** v1/marketstatus/now | Market Status
*ReferenceApi* | [**v1_marketstatus_upcoming_get**](ReferenceApi.md#v1_marketstatus_upcoming_get) | **GET** v1/marketstatus/upcoming | Market Holidays
*ReferenceApi* | [**v1_meta_symbols_symbol_company_get**](ReferenceApi.md#v1_meta_symbols_symbol_company_get) | **GET** v1/meta/symbols/{symbol}/company | Ticker Details
*ReferenceApi* | [**v1_meta_symbols_symbol_news_get**](ReferenceApi.md#v1_meta_symbols_symbol_news_get) | **GET** v1/meta/symbols/{symbol}/news | Ticker News
*ReferenceApi* | [**v2_reference_dividends_symbol_get**](ReferenceApi.md#v2_reference_dividends_symbol_get) | **GET** v2/reference/dividends/{symbol} | Stock Dividends
*ReferenceApi* | [**v2_reference_financials_symbol_get**](ReferenceApi.md#v2_reference_financials_symbol_get) | **GET** v2/reference/financials/{symbol} | Stock Financials
*ReferenceApi* | [**v2_reference_locales_get**](ReferenceApi.md#v2_reference_locales_get) | **GET** v2/reference/locales | Locales
*ReferenceApi* | [**v2_reference_markets_get**](ReferenceApi.md#v2_reference_markets_get) | **GET** v2/reference/markets | Markets
*ReferenceApi* | [**v2_reference_splits_symbol_get**](ReferenceApi.md#v2_reference_splits_symbol_get) | **GET** v2/reference/splits/{symbol} | Stock Splits
*ReferenceApi* | [**v2_reference_tickers_get**](ReferenceApi.md#v2_reference_tickers_get) | **GET** v2/reference/tickers | Tickers
*ReferenceApi* | [**v2_reference_types_get**](ReferenceApi.md#v2_reference_types_get) | **GET** v2/reference/types | Ticker Types
*StocksEquitiesApi* | [**v1_last_quote_stocks_symbol_get**](StocksEquitiesApi.md#v1_last_quote_stocks_symbol_get) | **GET** v1/last_quote/stocks/{symbol} | Last Quote for a Symbol
*StocksEquitiesApi* | [**v1_last_stocks_symbol_get**](StocksEquitiesApi.md#v1_last_stocks_symbol_get) | **GET** v1/last/stocks/{symbol} | Last Trade for a Symbol
*StocksEquitiesApi* | [**v1_meta_conditions_ticktype_get**](StocksEquitiesApi.md#v1_meta_conditions_ticktype_get) | **GET** v1/meta/conditions/{ticktype} | Condition Mappings
*StocksEquitiesApi* | [**v1_meta_exchanges_get**](StocksEquitiesApi.md#v1_meta_exchanges_get) | **GET** v1/meta/exchanges | Exchanges
*StocksEquitiesApi* | [**v1_open_close_symbol_date_get**](StocksEquitiesApi.md#v1_open_close_symbol_date_get) | **GET** v1/open-close/{symbol}/{date} | Daily Open / Close
*StocksEquitiesApi* | [**v2_aggs_grouped_locale_locale_market_market_date_get**](StocksEquitiesApi.md#v2_aggs_grouped_locale_locale_market_market_date_get) | **GET** v2/aggs/grouped/locale/{locale}/market/{market}/{date} | Grouped Daily ( Bars )
*StocksEquitiesApi* | [**v2_aggs_ticker_ticker_prev_get**](StocksEquitiesApi.md#v2_aggs_ticker_ticker_prev_get) | **GET** v2/aggs/ticker/{ticker}/prev | Previous Close
*StocksEquitiesApi* | [**v2_aggs_ticker_ticker_range_multiplier_timespan_from_to_get**](StocksEquitiesApi.md#v2_aggs_ticker_ticker_range_multiplier_timespan_from_to_get) | **GET** v2/aggs/ticker/{ticker}/range/{multiplier}/{timespan}/{from}/{to} | Aggregates ( Bars )
*StocksEquitiesApi* | [**v2_snapshot_locale_us_markets_stocks_direction_get**](StocksEquitiesApi.md#v2_snapshot_locale_us_markets_stocks_direction_get) | **GET** v2/snapshot/locale/us/markets/stocks/{direction} | Snapshot - Gainers / Losers
*StocksEquitiesApi* | [**v2_snapshot_locale_us_markets_stocks_tickers_get**](StocksEquitiesApi.md#v2_snapshot_locale_us_markets_stocks_tickers_get) | **GET** v2/snapshot/locale/us/markets/stocks/tickers | Snapshot - All Tickers
*StocksEquitiesApi* | [**v2_snapshot_locale_us_markets_stocks_tickers_ticker_get**](StocksEquitiesApi.md#v2_snapshot_locale_us_markets_stocks_tickers_ticker_get) | **GET** v2/snapshot/locale/us/markets/stocks/tickers/{ticker} | Snapshot - Single Ticker
*StocksEquitiesApi* | [**v2_ticks_stocks_nbbo_ticker_date_get**](StocksEquitiesApi.md#v2_ticks_stocks_nbbo_ticker_date_get) | **GET** v2/ticks/stocks/nbbo/{ticker}/{date} | Historic Quotes ( NBBO )
*StocksEquitiesApi* | [**v2_ticks_stocks_trades_ticker_date_get**](StocksEquitiesApi.md#v2_ticks_stocks_trades_ticker_date_get) | **GET** v2/ticks/stocks/trades/{ticker}/{date} | Historic Trades

## Documentation For Models

 - [AggResponse](AggResponse.md)
 - [Aggregate](Aggregate.md)
 - [Aggv2](Aggv2.md)
 - [AnalystRatings](AnalystRatings.md)
 - [Company](Company.md)
 - [ConditionTypeMap](ConditionTypeMap.md)
 - [Conflict](Conflict.md)
 - [CryptoExchange](CryptoExchange.md)
 - [CryptoSnapshotAgg](CryptoSnapshotAgg.md)
 - [CryptoSnapshotBookItem](CryptoSnapshotBookItem.md)
 - [CryptoSnapshotTicker](CryptoSnapshotTicker.md)
 - [CryptoSnapshotTickerBook](CryptoSnapshotTickerBook.md)
 - [CryptoTick](CryptoTick.md)
 - [CryptoTickJson](CryptoTickJson.md)
 - [Dividend](Dividend.md)
 - [Earning](Earning.md)
 - [Error](Error.md)
 - [Exchange](Exchange.md)
 - [Financial](Financial.md)
 - [Financials](Financials.md)
 - [Forex](Forex.md)
 - [ForexAggregate](ForexAggregate.md)
 - [ForexSnapshotAgg](ForexSnapshotAgg.md)
 - [ForexSnapshotTicker](ForexSnapshotTicker.md)
 - [HistTrade](HistTrade.md)
 - [InlineResponse200](InlineResponse200.md)
 - [InlineResponse2001](InlineResponse2001.md)
 - [InlineResponse20010](InlineResponse20010.md)
 - [InlineResponse20011](InlineResponse20011.md)
 - [InlineResponse20012](InlineResponse20012.md)
 - [InlineResponse20013](InlineResponse20013.md)
 - [InlineResponse20014](InlineResponse20014.md)
 - [InlineResponse20015](InlineResponse20015.md)
 - [InlineResponse20016](InlineResponse20016.md)
 - [InlineResponse20017](InlineResponse20017.md)
 - [InlineResponse20017LastAverage](InlineResponse20017LastAverage.md)
 - [InlineResponse20018](InlineResponse20018.md)
 - [InlineResponse20019](InlineResponse20019.md)
 - [InlineResponse2002](InlineResponse2002.md)
 - [InlineResponse20020](InlineResponse20020.md)
 - [InlineResponse20021](InlineResponse20021.md)
 - [InlineResponse20022](InlineResponse20022.md)
 - [InlineResponse2002Results](InlineResponse2002Results.md)
 - [InlineResponse2003](InlineResponse2003.md)
 - [InlineResponse2003Results](InlineResponse2003Results.md)
 - [InlineResponse2004](InlineResponse2004.md)
 - [InlineResponse2005](InlineResponse2005.md)
 - [InlineResponse2006](InlineResponse2006.md)
 - [InlineResponse2007](InlineResponse2007.md)
 - [InlineResponse2008](InlineResponse2008.md)
 - [InlineResponse2009](InlineResponse2009.md)
 - [LastForexQuote](LastForexQuote.md)
 - [LastForexTrade](LastForexTrade.md)
 - [LastQuote](LastQuote.md)
 - [LastTrade](LastTrade.md)
 - [MarketHoliday](MarketHoliday.md)
 - [MarketStatus](MarketStatus.md)
 - [MarketStatusCurrencies](MarketStatusCurrencies.md)
 - [MarketStatusExchanges](MarketStatusExchanges.md)
 - [News](News.md)
 - [NotFound](NotFound.md)
 - [Quote](Quote.md)
 - [RatingSection](RatingSection.md)
 - [Split](Split.md)
 - [StocksOpenClose](StocksOpenClose.md)
 - [StocksSnapshotAgg](StocksSnapshotAgg.md)
 - [StocksSnapshotBookItem](StocksSnapshotBookItem.md)
 - [StocksSnapshotQuote](StocksSnapshotQuote.md)
 - [StocksSnapshotTicker](StocksSnapshotTicker.md)
 - [StocksSnapshotTickerBook](StocksSnapshotTickerBook.md)
 - [StocksV2Nbbo](StocksV2Nbbo.md)
 - [StocksV2Trade](StocksV2Trade.md)
 - [Symbol](Symbol.md)
 - [SymbolTypeMap](SymbolTypeMap.md)
 - [Ticker](Ticker.md)
 - [TickerCodes](TickerCodes.md)
 - [Trade](Trade.md)
 - [Unauthorized](Unauthorized.md)

## Documentation For Authorization

## apiKey
- **Type**: API key 

Example
```
	auth := context.WithValue(context.TODO(), sw.ContextAPIKey, sw.APIKey{
		Key: "APIKEY",
		Prefix: "Bearer", // Omit if not necessary.
	})
    r, err := client.Service.Operation(auth, args)
```

## Author


