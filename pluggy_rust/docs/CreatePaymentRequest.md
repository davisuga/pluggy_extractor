# CreatePaymentRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**amount** | **f64** | Requested amount | 
**description** | Option<**String**> | Payment description | [optional]
**callback_urls** | Option<[**models::PaymentRequestCallbackUrls**](PaymentRequestCallbackUrls.md)> |  | [optional]
**recipient_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Payment receiver identifier | [optional]
**customer_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Customer identifier associated to the payment | [optional]
**client_payment_id** | Option<**String**> | Your payment identifier | [optional]
**schedule** | Option<**String**> |  | [optional]
**is_sandbox** | Option<**bool**> | Indicates if this payment request should be created in sandbox mode. Default: false. | [optional][default to false]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


