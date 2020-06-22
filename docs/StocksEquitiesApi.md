# {{classname}}

All URIs are relative to *https://api.polygon.io/*

Method | HTTP request | Description
------------- | ------------- | -------------
[**v1_last_quote_stocks_symbol_get**](StocksEquitiesApi.md#v1_last_quote_stocks_symbol_get) | **GET** /v1/last_quote/stocks/{symbol} | Last Quote for a Symbol
[**v1_last_stocks_symbol_get**](StocksEquitiesApi.md#v1_last_stocks_symbol_get) | **GET** /v1/last/stocks/{symbol} | Last Trade for a Symbol
[**v1_meta_conditions_ticktype_get**](StocksEquitiesApi.md#v1_meta_conditions_ticktype_get) | **GET** /v1/meta/conditions/{ticktype} | Condition Mappings
[**v1_meta_exchanges_get**](StocksEquitiesApi.md#v1_meta_exchanges_get) | **GET** /v1/meta/exchanges | Exchanges
[**v1_open_close_symbol_date_get**](StocksEquitiesApi.md#v1_open_close_symbol_date_get) | **GET** /v1/open-close/{symbol}/{date} | Daily Open / Close
[**v2_aggs_grouped_locale_locale_market_market_date_get**](StocksEquitiesApi.md#v2_aggs_grouped_locale_locale_market_market_date_get) | **GET** /v2/aggs/grouped/locale/{locale}/market/{market}/{date} | Grouped Daily ( Bars )
[**v2_aggs_ticker_ticker_prev_get**](StocksEquitiesApi.md#v2_aggs_ticker_ticker_prev_get) | **GET** /v2/aggs/ticker/{ticker}/prev | Previous Close
[**v2_aggs_ticker_ticker_range_multiplier_timespan_from_to_get**](StocksEquitiesApi.md#v2_aggs_ticker_ticker_range_multiplier_timespan_from_to_get) | **GET** /v2/aggs/ticker/{ticker}/range/{multiplier}/{timespan}/{from}/{to} | Aggregates ( Bars )
[**v2_snapshot_locale_us_markets_stocks_direction_get**](StocksEquitiesApi.md#v2_snapshot_locale_us_markets_stocks_direction_get) | **GET** /v2/snapshot/locale/us/markets/stocks/{direction} | Snapshot - Gainers / Losers
[**v2_snapshot_locale_us_markets_stocks_tickers_get**](StocksEquitiesApi.md#v2_snapshot_locale_us_markets_stocks_tickers_get) | **GET** /v2/snapshot/locale/us/markets/stocks/tickers | Snapshot - All Tickers
[**v2_snapshot_locale_us_markets_stocks_tickers_ticker_get**](StocksEquitiesApi.md#v2_snapshot_locale_us_markets_stocks_tickers_ticker_get) | **GET** /v2/snapshot/locale/us/markets/stocks/tickers/{ticker} | Snapshot - Single Ticker
[**v2_ticks_stocks_nbbo_ticker_date_get**](StocksEquitiesApi.md#v2_ticks_stocks_nbbo_ticker_date_get) | **GET** /v2/ticks/stocks/nbbo/{ticker}/{date} | Historic Quotes ( NBBO )
[**v2_ticks_stocks_trades_ticker_date_get**](StocksEquitiesApi.md#v2_ticks_stocks_trades_ticker_date_get) | **GET** /v2/ticks/stocks/trades/{ticker}/{date} | Historic Trades

# **v1_last_quote_stocks_symbol_get**
> InlineResponse2009 v1_last_quote_stocks_symbol_get(ctx, symbol)
Last Quote for a Symbol

Get the last quote tick for a given stock. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **symbol** | **String**| Symbol of the quote to get | 

### Return type

[**InlineResponse2009**](inline_response_200_9.md)

### Authorization

[apiKey](../README.md#apiKey), 

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **v1_last_stocks_symbol_get**
> InlineResponse2008 v1_last_stocks_symbol_get(ctx, symbol)
Last Trade for a Symbol

Get the last trade for a given stock. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **symbol** | **String**| Symbol of the stock to get | 

### Return type

[**InlineResponse2008**](inline_response_200_8.md)

### Authorization

[apiKey](../README.md#apiKey), 

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **v1_meta_conditions_ticktype_get**
> ConditionTypeMap v1_meta_conditions_ticktype_get(ctx, ticktype)
Condition Mappings

The mappings for conditions on trades and quotes. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **ticktype** | **String**| Ticker type we want mappings for  | 

### Return type

[**ConditionTypeMap**](ConditionTypeMap.md)

### Authorization

[apiKey](../README.md#apiKey), 

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **v1_meta_exchanges_get**
> Vec<Exchange> v1_meta_exchanges_get(ctx, )
Exchanges

List of stock exchanges which are supported by Polygon.io 

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**Vec<Exchange>**](Exchange.md)

### Authorization

[apiKey](../README.md#apiKey), 

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **v1_open_close_symbol_date_get**
> StocksOpenClose v1_open_close_symbol_date_get(ctx, symbol, date)
Daily Open / Close

Get the open, close and afterhours prices of a symbol on a certain date. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **symbol** | **String**| Symbol of the stock to get | 
  **date** | **String**| Date of the requested open/close ( YYYY-MM-DD format ) | 

### Return type

[**StocksOpenClose**](StocksOpenClose.md)

### Authorization

[apiKey](../README.md#apiKey), 

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **v2_aggs_grouped_locale_locale_market_market_date_get**
> AggResponse v2_aggs_grouped_locale_locale_market_market_date_get(ctx, locale, market, date, optional)
Grouped Daily ( Bars )

Get the daily OHLC for entire markets.  ### *** Warning, may cause browser to hang *** The response size is large, and sometimes will cause the browser prettyprint to crash. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **locale** | **String**| Locale of the aggregates ( See &#x27;Locales&#x27; API ) | 
  **market** | **String**| Market of the aggregates ( See &#x27;Markets&#x27; API ) | 
  **date** | **String**| To date | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **locale** | **String**| Locale of the aggregates ( See &#x27;Locales&#x27; API ) | 
 **market** | **String**| Market of the aggregates ( See &#x27;Markets&#x27; API ) | 
 **date** | **String**| To date | 
 **unadjusted** | **bool**| Set to true if the results should NOT be adjusted for splits.  | 

### Return type

[**AggResponse**](AggResponse.md)

### Authorization

[apiKey](../README.md#apiKey), 

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **v2_aggs_ticker_ticker_prev_get**
> AggResponse v2_aggs_ticker_ticker_prev_get(ctx, ticker, optional)
Previous Close

Get the previous day close for the specified ticker 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **ticker** | **String**| Ticker symbol of the request | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ticker** | **String**| Ticker symbol of the request | 
 **unadjusted** | **bool**| Set to true if the results should NOT be adjusted for splits.  | 

### Return type

[**AggResponse**](AggResponse.md)

### Authorization

[apiKey](../README.md#apiKey), 

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **v2_aggs_ticker_ticker_range_multiplier_timespan_from_to_get**
> AggResponse v2_aggs_ticker_ticker_range_multiplier_timespan_from_to_get(ctx, ticker, multiplier, timespan, from, to, optional)
Aggregates ( Bars )

Get aggregates for a date range, in custom time window sizes.  Some tickers require a prefix, see examples below&amp;#58; - Stocks&amp;#58; **AAPL** - Currencies&amp;#58; **C&amp;#58;EURUSD** - Crypto&amp;#58; **X&amp;#58;BTCUSD** 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **ticker** | **String**| Ticker symbol of the request | 
  **multiplier** | **f32**| Size of the timespan multiplier | 
  **timespan** | **String**| Size of the time window | 
  **from** | **String**| From date | 
  **to** | **String**| To date | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ticker** | **String**| Ticker symbol of the request | 
 **multiplier** | **f32**| Size of the timespan multiplier | 
 **timespan** | **String**| Size of the time window | 
 **from** | **String**| From date | 
 **to** | **String**| To date | 
 **unadjusted** | **bool**| Set to true if the results should NOT be adjusted for splits.  | 
 **sort** | [**String**](.md)| sort by timestamp  | [default to asc]

### Return type

[**AggResponse**](AggResponse.md)

### Authorization

[apiKey](../README.md#apiKey), 

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **v2_snapshot_locale_us_markets_stocks_direction_get**
> InlineResponse20010 v2_snapshot_locale_us_markets_stocks_direction_get(ctx, direction)
Snapshot - Gainers / Losers

See the current snapshot of the top 20 gainers or losers of the day at the moment. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **direction** | **String**| Direction we want  | 

### Return type

[**InlineResponse20010**](inline_response_200_10.md)

### Authorization

[apiKey](../README.md#apiKey), 

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **v2_snapshot_locale_us_markets_stocks_tickers_get**
> InlineResponse20010 v2_snapshot_locale_us_markets_stocks_tickers_get(ctx, )
Snapshot - All Tickers

Snapshot allows you to see all tickers current minute aggregate, daily aggregate and last trade. As well as previous days aggregate and calculated change for today.  ### *** Warning, may cause browser to hang *** The response size is large, and sometimes will cause the browser prettyprint to crash. 

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**InlineResponse20010**](inline_response_200_10.md)

### Authorization

[apiKey](../README.md#apiKey), 

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **v2_snapshot_locale_us_markets_stocks_tickers_ticker_get**
> InlineResponse20011 v2_snapshot_locale_us_markets_stocks_tickers_ticker_get(ctx, ticker)
Snapshot - Single Ticker

See the current snapshot of a single ticker 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **ticker** | **String**| Ticker of the snapshot | 

### Return type

[**InlineResponse20011**](inline_response_200_11.md)

### Authorization

[apiKey](../README.md#apiKey), 

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **v2_ticks_stocks_nbbo_ticker_date_get**
> InlineResponse2007 v2_ticks_stocks_nbbo_ticker_date_get(ctx, ticker, date, optional)
Historic Quotes ( NBBO )

Get historic NBBO quotes for a ticker. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **ticker** | **String**| Ticker symbol we want ticks for | 
  **date** | **String**| Date/Day of the historic ticks to retrieve ( YYYY-MM-DD ) | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ticker** | **String**| Ticker symbol we want ticks for | 
 **date** | **String**| Date/Day of the historic ticks to retrieve ( YYYY-MM-DD ) | 
 **timestamp** | **i64**| Timestamp offset, used for pagination. This is the offset at which to start the results. Using the &#x60;timestamp&#x60; of the last result as the offset will give you the next page of results.  | 
 **timestamp_limit** | **i64**| Maximum timestamp allowed in the results.  | 
 **reverse** | **bool**| Reverse the order of the results. This is useful in combination with &#x60;timestamp&#x60; param.  | 
 **limit** | **i64**| Limit the size of response, Max 50000 | [default to 10]

### Return type

[**InlineResponse2007**](inline_response_200_7.md)

### Authorization

[apiKey](../README.md#apiKey), 

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **v2_ticks_stocks_trades_ticker_date_get**
> InlineResponse2006 v2_ticks_stocks_trades_ticker_date_get(ctx, ticker, date, optional)
Historic Trades

Get historic trades for a ticker. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **ticker** | **String**| Ticker symbol we want ticks for | 
  **date** | **String**| Date/Day of the historic ticks to retrieve ( YYYY-MM-DD ) | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ticker** | **String**| Ticker symbol we want ticks for | 
 **date** | **String**| Date/Day of the historic ticks to retrieve ( YYYY-MM-DD ) | 
 **timestamp** | **i64**| Timestamp offset, used for pagination. This is the offset at which to start the results. Using the &#x60;timestamp&#x60; of the last result as the offset will give you the next page of results.  | 
 **timestamp_limit** | **i64**| Maximum timestamp allowed in the results.  | 
 **reverse** | **bool**| Reverse the order of the results. This is useful in combination with &#x60;timestamp&#x60; param.  | 
 **limit** | **i64**| Limit the size of response, Max 50000 | [default to 10]

### Return type

[**InlineResponse2006**](inline_response_200_6.md)

### Authorization

[apiKey](../README.md#apiKey), 

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

