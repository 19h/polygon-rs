# {{classname}}

All URIs are relative to *https://api.polygon.io/*

Method | HTTP request | Description
------------- | ------------- | -------------
[**v1_historic_crypto_from_to_date_get**](CryptoApi.md#v1_historic_crypto_from_to_date_get) | **GET** /v1/historic/crypto/{from}/{to}/{date} | Historic Crypto Trades
[**v1_last_crypto_from_to_get**](CryptoApi.md#v1_last_crypto_from_to_get) | **GET** /v1/last/crypto/{from}/{to} | Last Trade for a Crypto Pair
[**v1_meta_crypto_exchanges_get**](CryptoApi.md#v1_meta_crypto_exchanges_get) | **GET** /v1/meta/crypto-exchanges | Crypto Exchanges
[**v1_open_close_crypto_from_to_date_get**](CryptoApi.md#v1_open_close_crypto_from_to_date_get) | **GET** /v1/open-close/crypto/{from}/{to}/{date} | Daily Open / Close
[**v2_aggs_grouped_locale_locale_market_market_date_get**](CryptoApi.md#v2_aggs_grouped_locale_locale_market_market_date_get) | **GET** /v2/aggs/grouped/locale/{locale}/market/{market}/{date} | Grouped Daily ( Bars )
[**v2_aggs_ticker_ticker_prev_get**](CryptoApi.md#v2_aggs_ticker_ticker_prev_get) | **GET** /v2/aggs/ticker/{ticker}/prev | Previous Close
[**v2_aggs_ticker_ticker_range_multiplier_timespan_from_to_get**](CryptoApi.md#v2_aggs_ticker_ticker_range_multiplier_timespan_from_to_get) | **GET** /v2/aggs/ticker/{ticker}/range/{multiplier}/{timespan}/{from}/{to} | Aggregates ( Bars )
[**v2_snapshot_locale_global_markets_crypto_direction_get**](CryptoApi.md#v2_snapshot_locale_global_markets_crypto_direction_get) | **GET** /v2/snapshot/locale/global/markets/crypto/{direction} | Snapshot - Gainers / Losers
[**v2_snapshot_locale_global_markets_crypto_tickers_get**](CryptoApi.md#v2_snapshot_locale_global_markets_crypto_tickers_get) | **GET** /v2/snapshot/locale/global/markets/crypto/tickers | Snapshot - All Tickers
[**v2_snapshot_locale_global_markets_crypto_tickers_ticker_book_get**](CryptoApi.md#v2_snapshot_locale_global_markets_crypto_tickers_ticker_book_get) | **GET** /v2/snapshot/locale/global/markets/crypto/tickers/{ticker}/book | Snapshot - Single Ticker Full Book ( L2 )
[**v2_snapshot_locale_global_markets_crypto_tickers_ticker_get**](CryptoApi.md#v2_snapshot_locale_global_markets_crypto_tickers_ticker_get) | **GET** /v2/snapshot/locale/global/markets/crypto/tickers/{ticker} | Snapshot - Single Ticker

# **v1_historic_crypto_from_to_date_get**
> InlineResponse20017 v1_historic_crypto_from_to_date_get(ctx, from, to, date, optional)
Historic Crypto Trades

Get historic trade ticks for a crypto pair. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **from** | **String**| From Symbol of the crypto pair | 
  **to** | **String**| To Symbol of the crypto pair | 
  **date** | **String**| Date/Day of the historic ticks to retrieve | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **from** | **String**| From Symbol of the crypto pair | 
 **to** | **String**| To Symbol of the crypto pair | 
 **date** | **String**| Date/Day of the historic ticks to retrieve | 
 **offset** | **i64**| Timestamp offset, used for pagination. This is the offset at which to start the results. Using the &#x60;timestamp&#x60; of the last result as the offset will give you the next page of results.  | 
 **limit** | **i64**| Limit the size of response, Max 10000 | [default to 100]

### Return type

[**InlineResponse20017**](inline_response_200_17.md)

### Authorization

[apiKey](../README.md#apiKey), 

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **v1_last_crypto_from_to_get**
> InlineResponse20015 v1_last_crypto_from_to_get(ctx, from, to)
Last Trade for a Crypto Pair

Get Last Trade Tick for a Currency Pair. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **from** | **String**| From Symbol of the pair | 
  **to** | **String**| To Symbol of the pair | 

### Return type

[**InlineResponse20015**](inline_response_200_15.md)

### Authorization

[apiKey](../README.md#apiKey), 

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **v1_meta_crypto_exchanges_get**
> Vec<CryptoExchange> v1_meta_crypto_exchanges_get(ctx, )
Crypto Exchanges

List of crypto currency exchanges which are supported by Polygon.io 

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**Vec<CryptoExchange>**](CryptoExchange.md)

### Authorization

[apiKey](../README.md#apiKey), 

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **v1_open_close_crypto_from_to_date_get**
> InlineResponse20016 v1_open_close_crypto_from_to_date_get(ctx, from, to, date)
Daily Open / Close

Get the open, close prices of a symbol on a certain day. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **from** | **String**| From Symbol of the pair | 
  **to** | **String**| To Symbol of the pair | 
  **date** | **String**| Date of the requested open/close | 

### Return type

[**InlineResponse20016**](inline_response_200_16.md)

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

Get aggregates for a date range, in custom time window sizes.  Some tickers require a prefix, see examples below&#58; - Stocks&#58; **AAPL** - Currencies&#58; **C&#58;EURUSD** - Crypto&#58; **X&#58;BTCUSD** 

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

# **v2_snapshot_locale_global_markets_crypto_direction_get**
> InlineResponse20018 v2_snapshot_locale_global_markets_crypto_direction_get(ctx, direction)
Snapshot - Gainers / Losers

See the current snapshot of the top 20 gainers or losers of the day at the moment. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **direction** | **String**| Direction we want  | 

### Return type

[**InlineResponse20018**](inline_response_200_18.md)

### Authorization

[apiKey](../README.md#apiKey), 

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **v2_snapshot_locale_global_markets_crypto_tickers_get**
> InlineResponse20018 v2_snapshot_locale_global_markets_crypto_tickers_get(ctx, )
Snapshot - All Tickers

Snapshot allows you to see all tickers current minute aggregate, daily aggregate and last trade. As well as previous days aggregate and calculated change for today.  ### *** Warning, may cause browser to hang *** The response size is large, and sometimes will cause the browser prettyprint to crash. 

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**InlineResponse20018**](inline_response_200_18.md)

### Authorization

[apiKey](../README.md#apiKey), 

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **v2_snapshot_locale_global_markets_crypto_tickers_ticker_book_get**
> InlineResponse20020 v2_snapshot_locale_global_markets_crypto_tickers_ticker_book_get(ctx, ticker)
Snapshot - Single Ticker Full Book ( L2 )

See the current level 2 book of a single ticker. This is the combined book from all the exchanges. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **ticker** | **String**| Ticker of the snapshot | 

### Return type

[**InlineResponse20020**](inline_response_200_20.md)

### Authorization

[apiKey](../README.md#apiKey), 

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **v2_snapshot_locale_global_markets_crypto_tickers_ticker_get**
> InlineResponse20019 v2_snapshot_locale_global_markets_crypto_tickers_ticker_get(ctx, ticker)
Snapshot - Single Ticker

See the current snapshot of a single ticker 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **ticker** | **String**| Ticker of the snapshot | 

### Return type

[**InlineResponse20019**](inline_response_200_19.md)

### Authorization

[apiKey](../README.md#apiKey), 

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

