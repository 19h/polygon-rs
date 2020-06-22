# {{classname}}

All URIs are relative to *https://api.polygon.io/*

Method | HTTP request | Description
------------- | ------------- | -------------
[**v1_marketstatus_now_get**](ReferenceApi.md#v1_marketstatus_now_get) | **GET** v1/marketstatus/now | Market Status
[**v1_marketstatus_upcoming_get**](ReferenceApi.md#v1_marketstatus_upcoming_get) | **GET** v1/marketstatus/upcoming | Market Holidays
[**v1_meta_symbols_symbol_company_get**](ReferenceApi.md#v1_meta_symbols_symbol_company_get) | **GET** v1/meta/symbols/{symbol}/company | Ticker Details
[**v1_meta_symbols_symbol_news_get**](ReferenceApi.md#v1_meta_symbols_symbol_news_get) | **GET** v1/meta/symbols/{symbol}/news | Ticker News
[**v2_reference_dividends_symbol_get**](ReferenceApi.md#v2_reference_dividends_symbol_get) | **GET** v2/reference/dividends/{symbol} | Stock Dividends
[**v2_reference_financials_symbol_get**](ReferenceApi.md#v2_reference_financials_symbol_get) | **GET** v2/reference/financials/{symbol} | Stock Financials
[**v2_reference_locales_get**](ReferenceApi.md#v2_reference_locales_get) | **GET** v2/reference/locales | Locales
[**v2_reference_markets_get**](ReferenceApi.md#v2_reference_markets_get) | **GET** v2/reference/markets | Markets
[**v2_reference_splits_symbol_get**](ReferenceApi.md#v2_reference_splits_symbol_get) | **GET** v2/reference/splits/{symbol} | Stock Splits
[**v2_reference_tickers_get**](ReferenceApi.md#v2_reference_tickers_get) | **GET** v2/reference/tickers | Tickers
[**v2_reference_types_get**](ReferenceApi.md#v2_reference_types_get) | **GET** v2/reference/types | Ticker Types

# **v1_marketstatus_now_get**
> MarketStatus v1_marketstatus_now_get(ctx, )
Market Status

Current status of each market 

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**MarketStatus**](MarketStatus.md)

### Authorization

[apiKey](../README.md#apiKey), 

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **v1_marketstatus_upcoming_get**
> Vec<MarketHoliday> v1_marketstatus_upcoming_get(ctx, )
Market Holidays

Get upcoming market holidays and their open/close times 

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**Vec<MarketHoliday>**](MarketHoliday.md)

### Authorization

[apiKey](../README.md#apiKey), 

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **v1_meta_symbols_symbol_company_get**
> Company v1_meta_symbols_symbol_company_get(ctx, symbol)
Ticker Details

Get the details of the symbol company/entity. These are important details which offer an overview of the entity. Things like name, sector, description, logo and similar companies. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **symbol** | **String**| Symbol we want details for  | 

### Return type

[**Company**](Company.md)

### Authorization

[apiKey](../README.md#apiKey), 

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **v1_meta_symbols_symbol_news_get**
> Vec<News> v1_meta_symbols_symbol_news_get(ctx, symbol, optional)
Ticker News

Get news articles for this ticker. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **symbol** | **String**| Ticker we want details for  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **symbol** | **String**| Ticker we want details for  | 
 **perpage** | **f32**| How many items to be on each page during pagination. Max 50  | [default to 50.0]
 **page** | **f32**| Which page of results to return  | [default to 1.0]

### Return type

[**Vec<News>**](News.md)

### Authorization

[apiKey](../README.md#apiKey), 

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **v2_reference_dividends_symbol_get**
> InlineResponse2004 v2_reference_dividends_symbol_get(ctx, symbol)
Stock Dividends

Get the historical divdends for this ticker. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **symbol** | **String**| Symbol we want dividends for  | 

### Return type

[**InlineResponse2004**](inline_response_200_4.md)

### Authorization

[apiKey](../README.md#apiKey), 

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **v2_reference_financials_symbol_get**
> InlineResponse2005 v2_reference_financials_symbol_get(ctx, symbol, optional)
Stock Financials

Get the historical financials for this ticker. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **symbol** | **String**| Symbol we want financials for  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **symbol** | **String**| Symbol we want financials for  | 
 **limit** | **f32**| Limit the number of results  | [default to 5.0]
 **_type** | **String**| Type of reports  | 
 **sort** | **String**| Sort direction  | 

### Return type

[**InlineResponse2005**](inline_response_200_5.md)

### Authorization

[apiKey](../README.md#apiKey), 

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **v2_reference_locales_get**
> InlineResponse2002 v2_reference_locales_get(ctx, )
Locales

Get the list of currently supported locales 

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**InlineResponse2002**](inline_response_200_2.md)

### Authorization

[apiKey](../README.md#apiKey), 

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **v2_reference_markets_get**
> InlineResponse2001 v2_reference_markets_get(ctx, )
Markets

Get the list of currently supported markets 

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**InlineResponse2001**](inline_response_200_1.md)

### Authorization

[apiKey](../README.md#apiKey), 

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **v2_reference_splits_symbol_get**
> InlineResponse2003 v2_reference_splits_symbol_get(ctx, symbol)
Stock Splits

Get the historical splits for this symbol. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **symbol** | **String**| Symbol we want details for  | 

### Return type

[**InlineResponse2003**](inline_response_200_3.md)

### Authorization

[apiKey](../README.md#apiKey), 

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **v2_reference_tickers_get**
> Vec<Symbol> v2_reference_tickers_get(ctx, optional)
Tickers

Query all ticker symbols which are supported by Polygon.io. This API includes Indices, Crypto, FX, and Stocks/Equities. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **sort** | **String**| Which field to sort by.  For desc place a &#x60;-&#x60; in front of the field name.  **Example:** - &#x60;?sort&#x3D;-ticker&#x60; to sort symbols Z-A - &#x60;?sort&#x3D;type&#x60; to sort symbols by type  | [default to ticker]
 **_type** | **String**| If you want the results to only container a certain type.  **Example:** - &#x60;?type&#x3D;etp&#x60; to get all ETFs - &#x60;?type&#x3D;cs&#x60; to get all Common Stock&#x27;s  | 
 **market** | **String**| Get tickers for a specific market  **Example:** - &#x60;?market&#x3D;stocks&#x60; to get all stock tickers - &#x60;?market&#x3D;indices&#x60; to get all index tickers  | 
 **locale** | **String**| Get tickers for a specific region/locale  **Example:** - &#x60;?locale&#x3D;us&#x60; to get all US tickers - &#x60;?locale&#x3D;g&#x60; to get all Global tickers  | 
 **search** | **String**| Search the name of tickers  **Example:** - &#x60;?search&#x3D;microsoft&#x60; Search tickers for microsoft  | 
 **perpage** | **f32**| How many items to be on each page during pagination. Max 50  | [default to 50.0]
 **page** | **f32**| Which page of results to return  | [default to 1.0]
 **active** | **bool**| Filter for only active or inactive symbols  | 

### Return type

[**Vec<Symbol>**](Symbol.md)

### Authorization

[apiKey](../README.md#apiKey), 

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **v2_reference_types_get**
> InlineResponse200 v2_reference_types_get(ctx, )
Ticker Types

Get the mapping of ticker types to descriptions / long names 

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**InlineResponse200**](inline_response_200.md)

### Authorization

[apiKey](../README.md#apiKey), 

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

