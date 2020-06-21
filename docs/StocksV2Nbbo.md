# StocksV2Nbbo

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**T** | **String** | Ticker of the object | [optional] [default to null]
**t** | **i64** | Nanosecond accuracy SIP Unix Timestamp | [default to null]
**y** | **i64** | Nanosecond accuracy Participant/Exchange Unix Timestamp | [optional] [default to null]
**f** | **i64** | Nanosecond accuracy TRF(Trade Reporting Facility) Unix Timestamp | [optional] [default to null]
**q** | **i64** | Sequence Number | [default to null]
**c** | **Vec<i64>** | Conditions | [optional] [default to null]
**i** | **Vec<i64>** | Indicators | [optional] [default to null]
**p** | **i64** | BID Price | [default to null]
**x** | **i64** | BID Exchange ID | [default to null]
**s** | **i64** | BID Size ( In round lots ) | [default to null]
**P** | **i64** | ASK Price | [default to null]
**X** | **i64** | ASK Exchange ID | [default to null]
**S** | **i64** | ASK Size ( In round lots ) | [default to null]
**z** | **i64** | Tape where trade occurred. ( 1,2 &#x3D; CTA, 3 &#x3D; UTP ) | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

