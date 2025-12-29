# InvestmentTransaction

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Primary investment transaction identifier | 
**r#type** | **String** | Type of transactions | 
**movement_type** | Option<**String**> | Type of movement of the transaction | [optional]
**quantity** | Option<**f64**> | Quantity of the transaction | [optional]
**value** | Option<**f64**> | Value on the transaction's Date | [optional]
**amount** | Option<**f64**> | Gross amount of the operation. May be null only if type is TRANSFER | [optional]
**agreed_rate** | Option<**f64**> | Agreed rate for treasury applications | [optional]
**date** | **String** | Date when the transaction was made | 
**trade_date** | Option<**String**> | Date when the transaction was confirmed | [optional]
**expenses** | Option<[**models::InvestmentExpenses**](InvestmentExpenses.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


