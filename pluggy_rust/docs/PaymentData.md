# PaymentData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**payer** | Option<[**models::PaymentDataParticipant**](PaymentDataParticipant.md)> |  | [optional]
**receiver** | Option<[**models::PaymentDataParticipant**](PaymentDataParticipant.md)> |  | [optional]
**reason** | Option<**String**> | User's motive submitted while making the transfer | [optional]
**reference_number** | Option<**String**> | Reference number for the transfer/payment | [optional]
**receiver_reference_id** | Option<**String**> | String submitted by the receiver associated with the payment when generating the payment request. | [optional]
**payment_method** | Option<**String**> | Type of transfer. TED, DOC, PIX, TEV or BOLETO | [optional]
**boleto_metadata** | Option<[**models::PaymentDataBoletoMetadata**](PaymentDataBoletoMetadata.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


