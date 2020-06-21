# AggResponse

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ticker** | **String** | Ticker symbol requested | [default to null]
**status** | **String** | Status of the response | [default to null]
**adjusted** | **bool** | If this response was adjusted for splits | [default to null]
**query_count** | **f32** | Number of aggregate ( min or day ) used to generate the response | [optional] [default to null]
**results_count** | **f32** | Total number of results generated | [optional] [default to null]
**results** | [**Vec<Aggv2>**](Aggv2.md) |  | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

