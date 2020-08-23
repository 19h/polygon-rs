# StocksV2Trade

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**T** | **String** | Ticker of the object | [optional] [default to null]
**t** | **i64** | Nanosecond accuracy SIP Unix Timestamp | [default to null]
**y** | **i64** | Nanosecond accuracy Participant/Exchange Unix Timestamp | [optional] [default to null]
**f** | **i64** | Nanosecond accuracy TRF(Trade Reporting Facility) Unix Timestamp | [optional] [default to null]
**q** | **i64** | Sequence Number | [default to null]
**i** | **String** | Trade ID | [default to null]
**x** | **i64** | Exchange ID | [default to null]
**s** | **i64** | Size/Volume of the trade | [default to null]
**c** | **Vec<i64>** | Conditions | [optional] [default to null]
**p** | **f32** | Price of the trade | [optional] [default to null]
**z** | **i64** | Tape where trade occurred. ( 1,2 &#x3D; CTA, 3 &#x3D; UTP ) | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

