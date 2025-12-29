# CreatePaymentRecipient

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**tax_number** | **String** | Account owner tax number. Can be CPF or CNPJ (only numbers) | 
**name** | **String** | Account owner name. | 
**payment_institution_id** | [**uuid::Uuid**](uuid::Uuid.md) | Primary identifier of the institution associated to the payment recipient. | 
**account** | [**models::PaymentRecipientAccount**](PaymentRecipientAccount.md) | Recipient's bank account destination. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


