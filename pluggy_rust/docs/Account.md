# Account

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Primary account identifier | 
**r#type** | **String** | Type of account, may be BANK or CREDIT | 
**subtype** | **String** | Subtype of corresponding type of account | 
**number** | **String** | External identifier of the account | 
**name** | **String** | Name of the account in a descriptive format | 
**marketing_name** | Option<**String**> | Name of the account as defined externally | [optional]
**balance** | **f64** | Funds of the account | 
**item_id** | [**uuid::Uuid**](uuid::Uuid.md) | Attached item's primary identifier | 
**tax_number** | Option<**String**> | Tax ID of the corresponding owner | [optional]
**owner** | Option<**String**> | Name of the owner of the account | [optional]
**currency_code** | **String** | Code referencing the currency of the balance | 
**bank_data** | Option<[**models::BankData**](BankData.md)> |  | [optional]
**credit_data** | Option<[**models::CreditData**](CreditData.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


