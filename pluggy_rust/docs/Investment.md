# Investment

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Primary identifier | 
**item_id** | [**uuid::Uuid**](uuid::Uuid.md) | Identifier of the item linked to the investment | 
**r#type** | **String** | Investment Asset type | 
**subtype** | Option<**String**> | Investment subtype, depends on the type | [optional]
**number** | Option<**String**> | Reference number for this holder's asset | [optional]
**balance** | **f64** | The current net balance amount of the investment | 
**name** | **String** | Name on the provider | 
**last_month_rate** | Option<**f64**> | The performance rate of the investment in the last month | [optional]
**last_twelve_months_rate** | Option<**f64**> | The performance rate of the investment in the last 12 months | [optional]
**annual_rate** | Option<**f64**> | The performance rate of the investment in the last year | [optional]
**currency_code** | **String** | Currency ISO code for the amounts | 
**code** | Option<**String**> | Associated Code for the investment. For example, the code for a mutual fund is the CNPJ | [optional]
**isin** | Option<**String**> | 12-character ISIN, a globally unique identifier | [optional]
**value** | Option<**f64**> | Quota's current value at \"date\" | [optional]
**quantity** | Option<**f64**> | Quantity of quota at disposal | [optional]
**amount** | Option<**f64**> | Gross amount of the investment | [optional]
**taxes** | Option<**f64**> | Income taxes applied to the investment | [optional]
**taxes2** | Option<**f64**> | Financial taxes applied to the investment | [optional]
**date** | **String** | Value's quota date | 
**owner** | Option<**String**> | Owner/beneficiary associated with the investment | [optional]
**amount_profit** | Option<**f64**> | Profit/Loss to date over the investment | [optional]
**amount_withdrawal** | Option<**f64**> | The amount available to withdraw | [optional]
**amount_original** | Option<**f64**> | Amount originally invested | [optional]
**metadata** | Option<[**models::InvestmentMetadata**](InvestmentMetadata.md)> | Security Portability details | [optional]
**due_date** | Option<**String**> | Expiration Date | [optional]
**issuer** | Option<**String**> | The entity that issued the investment | [optional]
**issuer_cnpj** | Option<**String**> | The entity CNPJ that issued the investment | [optional]
**issue_date** | Option<**String**> | The date that the investment was issued | [optional]
**purchase_date** | Option<**String**> | The date that the investment was purchased | [optional]
**rate** | Option<**f64**> | Fixed rate percentage applied to the investment | [optional]
**rate_type** | Option<**String**> | Type of fixed-rate | [optional]
**fixed_annual_rate** | Option<**f64**> | Fixed income annual rate | [optional]
**status** | Option<**String**> | Current status of the investment enum value | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


