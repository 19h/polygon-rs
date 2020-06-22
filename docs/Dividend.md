# Dividend

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**symbol** | **StockSymbol** |  | [default to null]
**_type** | **String** | Refers to the dividend payment type&amp;lt;br/&amp;gt; - Dividend income&amp;lt;br/&amp;gt; - Interest income&amp;lt;br/&amp;gt; - Stock dividend&amp;lt;br/&amp;gt; - Short term capital gain&amp;lt;br/&amp;gt; - Medium term capital gain&amp;lt;br/&amp;gt; - Long term capital gain&amp;lt;br/&amp;gt; - Unspecified term capital gain&amp;lt;br/&amp;gt;  | [default to null]
**ex_date** | **DateTime<Utc>** | Execution date of the trade | [default to null]
**payment_date** | **DateTime<Utc>** | Payment date of the trade | [optional] [default to null]
**record_date** | **DateTime<Utc>** | Record date of the trade | [optional] [default to null]
**declared_date** | **DateTime<Utc>** | Declared date of the trade | [optional] [default to null]
**amount** | **f32** | Amount of the dividend | [default to null]
**qualified** | **String** | Refers to the dividend income type&amp;lt;br/&amp;gt; - P &#x3D; Partially qualified income&amp;lt;br/&amp;gt; - Q &#x3D; Qualified income&amp;lt;br/&amp;gt; - N &#x3D; Unqualified income&amp;lt;br/&amp;gt; - null &#x3D; N/A or unknown  | [optional] [default to QUALIFIED.NULL]
**flag** | **String** | Refers to the dividend flag, if set&amp;lt;br/&amp;gt; FI &#x3D; Final dividend, div ends or instrument ends&amp;lt;br/&amp;gt; LI &#x3D; Liquidation, instrument liquidates&amp;lt;br/&amp;gt; PR &#x3D; Proceeds of a sale of rights or shares&amp;lt;br/&amp;gt; RE &#x3D; Redemption of rights&amp;lt;br/&amp;gt; AC &#x3D; Accrued dividend&amp;lt;br/&amp;gt; AR &#x3D; Payment in arrears&amp;lt;br/&amp;gt; AD &#x3D; Additional payment&amp;lt;br/&amp;gt; EX &#x3D; Extra payment&amp;lt;br/&amp;gt; SP &#x3D; Special dividend&amp;lt;br/&amp;gt; YE &#x3D; Year end&amp;lt;br/&amp;gt; UR &#x3D; Unknown rate&amp;lt;br/&amp;gt; SU &#x3D; Regular dividend is suspended  | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

