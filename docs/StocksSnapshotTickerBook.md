# StocksSnapshotTickerBook

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ticker** | **String** | Ticker of the object | [default to null]
**bids** | [**Vec<StocksSnapshotBookItem>**](StocksSnapshotBookItem.md) | Bids | [optional] [default to null]
**asks** | [**Vec<StocksSnapshotBookItem>**](StocksSnapshotBookItem.md) | Asks | [optional] [default to null]
**bid_count** | **f32** | Combined total number of bids in the book | [optional] [default to null]
**ask_count** | **f32** | Combined total number of asks in the book | [optional] [default to null]
**spread** | **f32** | Difference between the best bid and the best ask price accross exchanges | [optional] [default to null]
**updated** | **i64** | Last Updated timestamp | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

