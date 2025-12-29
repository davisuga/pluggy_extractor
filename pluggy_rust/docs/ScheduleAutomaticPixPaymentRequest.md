# ScheduleAutomaticPixPaymentRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**amount** | **f64** | Transaction value | 
**description** | Option<**String**> | Transaction description | [optional]
**date** | [**String**](string.md) | The payment date, which must fall between D+2 and D+10. Date format must be YYYY-MM-DD (for example: 2025-06-16) | 
**client_payment_id** | Option<**String**> | External identifier for the payment | [optional]
**recipient_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Payment recipient identifier. It should be sent if you want to use a different recipient from the one consented in the payment request (it must have the same tax number as the consented recipient). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


