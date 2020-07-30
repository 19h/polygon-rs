# Split

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ticker** | **String** | An actual exchange traded ticker. | [default to null]
**ex_date** | **String** | Execution date of the split | [default to null]
**payment_date** | **String** | Payment date of the split | [default to null]
**record_date** | **String** | Payment date of the split | [optional] [default to null]
**declared_date** | **String** | Payment date of the split | [optional] [default to null]
**ratio** | **f32** | refers to the split ratio. The split ratio is an inverse of the number of shares that a holder of the stock would have after the split divided by the number of shares that the holder had before. &lt;br/&gt; For example: Split ratio of .5 &#x3D; 2 for 1 split.  | [default to null]
**tofactor** | **f32** | To factor of the split. Used to calculate the split ratio forfactor/tofactor &#x3D; ratio (eg ½ &#x3D; 0.5)  | [default to null]
**forfactor** | **f32** | For factor of the split. Used to calculate the split ratio forfactor/tofactor &#x3D; ratio (eg ½ &#x3D; 0.5)  | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

