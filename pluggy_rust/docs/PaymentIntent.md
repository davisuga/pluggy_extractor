# PaymentIntent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | Primary identifier | [optional]
**status** | Option<**String**> | Payment intent status | [optional]
**created_at** | Option<**String**> | Date when the payment intent was created | [optional]
**updated_at** | Option<**String**> | Date when the payment intent was updated | [optional]
**payment_request** | Option<[**models::PaymentRequest**](PaymentRequest.md)> | Payment request associated to the payment intent | [optional]
**connector** | Option<[**models::Connector**](Connector.md)> | Connector associated to the payment intent | [optional]
**consent_url** | Option<**String**> | Url to authorize the payment intent | [optional]
**reference_id** | Option<**String**> | Pix id related to the payment intent | [optional]
**payment_method** | Option<**String**> | Payment method can be PIS (Payment Initiation) or PIX | [optional][default to Pis]
**pix_data** | Option<[**models::PixData**](PixData.md)> | Pix data related to the payment intent (only applies for PIX payment method) | [optional]
**error_detail** | Option<[**models::PaymentIntentErrorDetail**](PaymentIntentErrorDetail.md)> | Error details when payment intent fails | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


