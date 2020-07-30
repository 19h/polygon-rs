# Ticker

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ticker** | **String** | An actual exchange symbol this item is traded under. | [default to null]
**name** | **String** | Name of the item. | [default to null]
**market** | **String** | The market in which this ticker participates | [default to null]
**locale** | **String** | Locale of where this ticker is traded | [default to null]
**currency** | **String** | Currency this ticker is traded in | [optional] [default to null]
**active** | **bool** | If the ticker is active. False means the ticker has been delisted | [optional] [default to null]
**primary_exch** | **String** | The listing exchange for this ticker | [optional] [default to null]
**url** | **String** | URL of this ticker. Use this to get more information about the ticker. | [optional] [default to null]
**updated** | **String** | Last time this ticker record was updated. | [default to null]
**attrs** | [***Value**](Value.md) | Additional details about this ticker. No schema. | [optional] [default to null]
**codes** | [***TickerCodes**](Ticker_codes.md) |  | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

