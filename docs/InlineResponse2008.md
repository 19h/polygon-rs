# InlineResponse2008

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**results_count** | **i64** | Total number of results in this response | [optional] [default to null]
**db_latency** | **i64** | Milliseconds of latency for the query results from DB | [optional] [default to null]
**success** | **bool** | If this query was executed successfully | [default to null]
**ticker** | **String** | Ticker symbol that was evaluated from the request | [default to null]
**results** | [**Vec<StocksV2Nbbo>**](StocksV2NBBO.md) |  | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

