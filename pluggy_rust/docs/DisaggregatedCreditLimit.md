# DisaggregatedCreditLimit

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**credit_line_limit_type** | **String** | Limit type (LIMITE_CREDITO_TOTAL or LIMITE_CREDITO_MODALIDADE_OPERACAO) | 
**consolidation_type** | **String** | Indicates if the limit is consolidated or individual | 
**identification_number** | **String** | Identification number of the additional credit card | 
**is_limit_flexible** | **bool** | Indicates if the limit is flexible | 
**used_amount** | **f64** | Used amount of the additional credit card | 
**used_amount_currency_code** | **String** | Used amount currency code (for example, BRL) | 
**line_name** | Option<**String**> | Name of the line (for example, 'Limite de Crédito') | [optional]
**line_name_additional_info** | Option<**String**> | Additional information about the line name | [optional]
**limit_amount** | Option<**f64**> | Limit amount of the additional credit card | [optional]
**limit_amount_currency_code** | Option<**String**> | Limit amount currency code (for example, BRL) | [optional]
**available_amount** | Option<**f64**> | Available amount of the additional credit card | [optional]
**available_amount_currency_code** | Option<**String**> | Available amount currency code (for example, BRL) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


