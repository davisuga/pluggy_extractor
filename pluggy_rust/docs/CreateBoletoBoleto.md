# CreateBoletoBoleto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**seu_numero** | **String** | Your identifier for this boleto | 
**amount** | **f64** | Boleto amount | 
**due_date** | **String** | Due date for the boleto. Must be today or in the future. | 
**payer** | [**models::CreateBoletoBoletoPayer**](CreateBoleto_boleto_payer.md) |  | 
**fine** | Option<[**models::CreateBoletoBoletoFine**](CreateBoleto_boleto_fine.md)> |  | [optional]
**interest** | Option<[**models::CreateBoletoBoletoInterest**](CreateBoleto_boleto_interest.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


