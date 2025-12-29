# CreateAutomaticPixPaymentRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**fixed_amount** | Option<**f64**> | Fixed charge amount; if filled in, it represents consent for payments of fixed amounts, not subject to change during the validity of the consent. If it's sent, minimumVariableAmount and maximumVariableAmount cannot be provided. | [optional]
**minimum_variable_amount** | Option<**f64**> | Minimum amount allowed per charge; if filled in, it represents consent for payments of variable amounts. If it's sent, fixedAmount cannot be provided. | [optional]
**maximum_variable_amount** | Option<**f64**> | Maximum amount allowed per charge; if filled in, it represents consent for payments of variable amounts. If it's sent, fixedAmount cannot be provided. | [optional]
**description** | Option<**String**> | Description for the automatic pix authorization | [optional]
**start_date** | [**String**](string.md) | Represents the expected date for the first occurrence of a payment associated with the recurrence. Date format must be YYYY-MM-DD (for example: 2025-06-16) | 
**expires_at** | Option<[**String**](string.md)> | Expiration date for the automatic pix authorization. The date must be in UTC and the format must follow the following pattern: YYYY-MM-DDTHH:MM:SSZ (for example: 2025-06-16T03:00:00Z). | [optional]
**is_retry_accepted** | Option<**bool**> | Indicates whether the receiving customer is allowed to make payment attempts, according to the rules established in the Pix arrangement. | [optional]
**first_payment** | Option<[**models::AutomaticPixFirstPayment**](AutomaticPixFirstPayment.md)> |  | [optional]
**interval** | **String** | Defines the permitted frequency for carrying out transactions. | 
**callback_urls** | Option<[**models::PaymentRequestCallbackUrls**](PaymentRequestCallbackUrls.md)> |  | [optional]
**recipient_id** | **String** | Primary identifier of the payment recipient | 
**client_payment_id** | Option<**String**> | Client payment identifier | [optional]
**customer_id** | Option<**String**> | Primary identifier of the customer | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


