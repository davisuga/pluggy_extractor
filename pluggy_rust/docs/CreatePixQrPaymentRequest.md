# CreatePixQrPaymentRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pix_qr_code** | **String** | Pix QR code | 
**callback_urls** | Option<[**models::PaymentRequestCallbackUrls**](PaymentRequestCallbackUrls.md)> |  | [optional]
**customer_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Customer identifier associated to the payment | [optional]
**is_sandbox** | Option<**bool**> | Indicates if this payment request should be created in sandbox mode. Default: false. | [optional][default to false]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


