# SmartTransferPayment

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Payment primary identifier | 
**preauthorization_id** | **String** | Payment primary identifier | 
**status** | **String** | Payment status | 
**amount** | **f64** | Payment amount | 
**description** | Option<**String**> | Payment description | [optional]
**recipient** | [**models::PaymentRecipient**](PaymentRecipient.md) |  | 
**client_payment_id** | Option<**String**> | Client payment identifier | [optional]
**created_at** | **String** | Date when the payemnt was created | 
**updated_at** | **String** | Date when the payment was updated | 
**error_detail** | Option<[**models::SmartTransferPreauthorizationErrorDetail**](SmartTransferPreauthorization_errorDetail.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


