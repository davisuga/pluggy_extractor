# CreatePaymentIntent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**payment_request_id** | Option<**String**> | Primary identifier of the payment request associated to the payment intent | [optional]
**parameters** | Option<[**models::PaymentIntentParameter**](PaymentIntentParameter.md)> |  | [optional]
**connector_id** | Option<**f64**> | Primary identifier of the connector associated to the payment intent | [optional]
**payment_method** | Option<**String**> | Payment method can be PIS (Payment Initiation) or PIX (PIX QR flow). | [optional]
**is_dynamic_pix** | Option<**bool**> | Only for PIX paymentMethod. If true, the generated PIX QR code is dynamic and one-use. This requires the customerId to be present, and the customer must have CPF/CNPJ | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


