# AutomaticPixPayment

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Payment primary identifier | 
**status** | **String** | Payment status | 
**amount** | **f64** | Payment amount | 
**description** | Option<**String**> | Payment description | [optional]
**date** | [**String**](string.md) | Payment scheduled date | 
**end_to_end_id** | Option<**String**> | Payment end to end identifier | [optional]
**error_detail** | Option<[**models::AutomaticPixPaymentErrorDetail**](AutomaticPixPaymentErrorDetail.md)> |  | [optional]
**client_payment_id** | Option<**String**> | External identifier for the payment | [optional]
**recipient_id** | [**uuid::Uuid**](uuid::Uuid.md) | Payment recipient identifier | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


