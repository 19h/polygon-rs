# Dividend

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**symbol** | **String** | An actual exchange symbol this item is traded under. | [default to null]
**_type** | **String** | Refers to the dividend payment type&lt;br/&gt; - Dividend income&lt;br/&gt; - Interest income&lt;br/&gt; - Stock dividend&lt;br/&gt; - Short term capital gain&lt;br/&gt; - Medium term capital gain&lt;br/&gt; - Long term capital gain&lt;br/&gt; - Unspecified term capital gain&lt;br/&gt;  | [default to null]
**ex_date** | **String** | Execution date of the trade | [default to null]
**payment_date** | **String** | Payment date of the trade | [optional] [default to null]
**record_date** | **String** | Record date of the trade | [optional] [default to null]
**declared_date** | **String** | Declared date of the trade | [optional] [default to null]
**amount** | **f32** | Amount of the dividend | [default to null]
**qualified** | **String** | Refers to the dividend income type&lt;br/&gt; - P &#x3D; Partially qualified income&lt;br/&gt; - Q &#x3D; Qualified income&lt;br/&gt; - N &#x3D; Unqualified income&lt;br/&gt; - null &#x3D; N/A or unknown  | [optional] [default to QUALIFIED.NULL]
**flag** | **String** | Refers to the dividend flag, if set&lt;br/&gt; FI &#x3D; Final dividend, div ends or instrument ends&lt;br/&gt; LI &#x3D; Liquidation, instrument liquidates&lt;br/&gt; PR &#x3D; Proceeds of a sale of rights or shares&lt;br/&gt; RE &#x3D; Redemption of rights&lt;br/&gt; AC &#x3D; Accrued dividend&lt;br/&gt; AR &#x3D; Payment in arrears&lt;br/&gt; AD &#x3D; Additional payment&lt;br/&gt; EX &#x3D; Extra payment&lt;br/&gt; SP &#x3D; Special dividend&lt;br/&gt; YE &#x3D; Year end&lt;br/&gt; UR &#x3D; Unknown rate&lt;br/&gt; SU &#x3D; Regular dividend is suspended  | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

