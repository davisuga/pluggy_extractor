# Boleto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**digitable_line** | **String** | Boleto digitable line | 
**barcode** | **String** | Boleto barcode | 
**payer** | [**models::BoletoPayer**](BoletoPayer.md) |  | 
**recipient** | [**models::BoletoRecipient**](BoletoRecipient.md) |  | 
**date** | Option<**String**> | Boleto issue date | [optional]
**due_date** | **String** | Boleto due date | 
**expiration_date** | Option<**String**> | After this date, the boleto cannot be paid | [optional]
**base_amount** | **f64** | Boleto original amount, without interests, penalties and discounts | 
**penalty_amount** | **f64** | Boleto penalty amount. If there is no penalty, it will be returned as zero | 
**interest_amount** | Option<**f64**> | Boleto interest amount. If there is no interest, it will be returned as zero | [optional]
**discount_amount** | **f64** | Boleto discount amount. If there is no discounts, it will be returned as zero | 
**total_amount** | **f64** | Boleto final amount. It is equal to the base amount plus penalties and interests, minus discounts | 
**updated_at** | Option<**String**> | Date when the lastest information of this boleto has been retrieved | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


