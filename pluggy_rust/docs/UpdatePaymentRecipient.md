# UpdatePaymentRecipient

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**tax_number** | Option<**String**> | Account owner tax number. Can be CPF or CNPJ (only numbers) | [optional]
**name** | Option<**String**> | Account owner name. | [optional]
**payment_institution_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Primary identifier of the institution associated to the payment recipient. | [optional]
**account** | Option<[**models::PaymentRecipientAccount**](PaymentRecipientAccount.md)> | Recipient's bank account destination. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


