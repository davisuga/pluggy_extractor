# SchedulePayment

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**description** | **String** | Scheduled payment description | 
**status** | **String** | Scheduled payment status | 
**scheduled_date** | [**String**](string.md) | Date when the payment is scheduled | 
**end_to_end_id** | Option<**String**> | Identifier for the payment, used to link the scheduled payment with the corresponding payment received | [optional]
**error_detail** | Option<[**models::SchedulePaymentErrorDetail**](SchedulePayment_errorDetail.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


