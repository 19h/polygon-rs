# {{classname}}

All URIs are relative to *https://api.polygon.io/*

Method | HTTP request | Description
------------- | ------------- | -------------
[**v1_conversion_from_to_get**](ForexCurrenciesApi.md#v1_conversion_from_to_get) | **GET** v1/conversion/{from}/{to} | Real-time Currency Conversion
[**v1_historic_forex_from_to_date_get**](ForexCurrenciesApi.md#v1_historic_forex_from_to_date_get) | **GET** v1/historic/forex/{from}/{to}/{date} | Historic Forex Ticks
[**v1_last_quote_currencies_from_to_get**](ForexCurrenciesApi.md#v1_last_quote_currencies_from_to_get) | **GET** v1/last_quote/currencies/{from}/{to} | Last Quote for a Currency Pair
[**v2_aggs_grouped_locale_locale_market_market_date_get**](ForexCurrenciesApi.md#v2_aggs_grouped_locale_locale_market_market_date_get) | **GET** v2/aggs/grouped/locale/{locale}/market/{market}/{date} | Grouped Daily ( Bars )
[**v2_aggs_ticker_ticker_prev_get**](ForexCurrenciesApi.md#v2_aggs_ticker_ticker_prev_get) | **GET** v2/aggs/ticker/{ticker}/prev | Previous Close
[**v2_aggs_ticker_ticker_range_multiplier_timespan_from_to_get**](ForexCurrenciesApi.md#v2_aggs_ticker_ticker_range_multiplier_timespan_from_to_get) | **GET** v2/aggs/ticker/{ticker}/range/{multiplier}/{timespan}/{from}/{to} | Aggregates ( Bars )
[**v2_snapshot_locale_global_markets_forex_direction_get**](ForexCurrenciesApi.md#v2_snapshot_locale_global_markets_forex_direction_get) | **GET** v2/snapshot/locale/global/markets/forex/{direction} | Snapshot - Gainers / Losers
[**v2_snapshot_locale_global_markets_forex_tickers_get**](ForexCurrenciesApi.md#v2_snapshot_locale_global_markets_forex_tickers_get) | **GET** v2/snapshot/locale/global/markets/forex/tickers | Snapshot - All Tickers

# **v1_conversion_from_to_get**
> InlineResponse20013 v1_conversion_from_to_get(ctx, from, to, optional)
Real-time Currency Conversion

Convert currencies using the latest market conversion rates. Note than you can convert in both directions. For example USD-&gt;CAD or CAD-&gt;USD. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **from** | **String**| From Symbol of the pair | 
  **to** | **String**| To Symbol of the pair | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **from** | **String**| From Symbol of the pair | 
 **to** | **String**| To Symbol of the pair | 
 **amount** | **i64**| Amount we want to convert. With decimal | [default to 100]
 **precision** | **i64**| Decimal precision of the conversion. Defaults to 2 which is 2 decimal places accuracy. | [default to 2]

### Return type

[**InlineResponse20013**](inline_response_200_13.md)

### Authorization

[apiKey](../README.md#apiKey), 

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **v1_historic_forex_from_to_date_get**
> InlineResponse20012 v1_historic_forex_from_to_date_get(ctx, from, to, date, optional)
Historic Forex Ticks

Get historic ticks for a currency pair. Example for **USD/JPY** thefrom would be **USD** and to would be **JPY**. The date formatted like **2017-6-22** 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **from** | **String**| From Symbol of the currency pair | 
  **to** | **String**| To Symbol of the currency pair | 
  **date** | **String**| Date/Day of the historic ticks to retrieve | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **from** | **String**| From Symbol of the currency pair | 
 **to** | **String**| To Symbol of the currency pair | 
 **date** | **String**| Date/Day of the historic ticks to retrieve | 
 **offset** | **i64**| Timestamp offset, used for pagination. This is the offset at which to start the results. Using the &#x60;timestamp&#x60; of the last result as the offset will give you the next page of results.  | 
 **limit** | **i64**| Limit the size of response, Max 10000 | [default to 100]

### Return type

[**InlineResponse20012**](inline_response_200_12.md)

### Authorization

[apiKey](../README.md#apiKey), 

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **v1_last_quote_currencies_from_to_get**
> InlineResponse20014 v1_last_quote_currencies_from_to_get(ctx, from, to)
Last Quote for a Currency Pair

Get Last Quote Tick for a Currency Pair. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **from** | **String**| From Symbol of the pair | 
  **to** | **String**| To Symbol of the pair | 

### Return type

[**InlineResponse20014**](inline_response_200_14.md)

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

# **v2_snapshot_locale_global_markets_forex_direction_get**
> InlineResponse20015 v2_snapshot_locale_global_markets_forex_direction_get(ctx, direction)
Snapshot - Gainers / Losers

See the current snapshot of the top 20 gainers or losers of the day at the moment. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **direction** | **String**| Direction we want  | 

### Return type

[**InlineResponse20015**](inline_response_200_15.md)

### Authorization

[apiKey](../README.md#apiKey), 

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **v2_snapshot_locale_global_markets_forex_tickers_get**
> InlineResponse20015 v2_snapshot_locale_global_markets_forex_tickers_get(ctx, )
Snapshot - All Tickers

Snapshot allows you to see all tickers current minute aggregate, daily aggregate and last trade. As well as previous days aggregate and calculated change for today.  ### *** Warning, may cause browser to hang *** The response size is large, and sometimes will cause the browser prettyprint to crash. 

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**InlineResponse20015**](inline_response_200_15.md)

### Authorization

[apiKey](../README.md#apiKey), 

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

