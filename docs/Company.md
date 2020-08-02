# Company

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**logo** | **String** | URL of the entities logo. | [optional] [default to null]
**exchange** | **String** | Symbols primary exchange | [default to null]
**name** | **String** | Name of the company/entity | [default to null]
**symbol** | **String** | An actual exchange symbol this item is traded under. | [default to null]
**listdate** | [***String**](string.md) | Date this symbol was listed on the exchange. | [optional] [default to null]
**cik** | **String** | Official CIK guid used for SEC database / filings. | [optional] [default to null]
**bloomberg** | **String** | Bloomberg guid for this symbol | [optional] [default to null]
**figi** | **String** | guid for the OpenFigi project ( https://openfigi.com/ ) | [optional] [default to null]
**lei** | **String** | Legal Entity Identifier (LEI) guid for symbol ( https://en.wikipedia.org/wiki/Legal_Entity_Identifier ) | [optional] [default to null]
**sic** | **i64** | Standard Industrial Classification (SIC) id for symbol ( https://en.wikipedia.org/wiki/Standard_Industrial_Classification ) | [optional] [default to null]
**country** | **String** | Country in which this company is registered | [optional] [default to null]
**industry** | **String** | Industry this company operates in | [optional] [default to null]
**sector** | **String** | Sector of the indsutry in which this symbol operates in | [optional] [default to null]
**marketcap** | **i64** | Current market cap for this company | [optional] [default to null]
**employees** | **i64** | Approximate number of employees | [optional] [default to null]
**phone** | **String** | Phone number for this company. Usually corporate contact number. | [optional] [default to null]
**ceo** | **String** | Name of the companies current CEO | [optional] [default to null]
**url** | **String** | URL of the companies website | [optional] [default to null]
**description** | **String** | A description of the company and what they do/offer | [default to null]
**similar** | **Vec<String>** |  | [optional] [default to null]
**tags** | **Vec<String>** |  | [optional] [default to null]
**updated** | **String** | Last time this company record was updated. | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

