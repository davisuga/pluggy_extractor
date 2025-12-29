# PaymentRecipient

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Primary identifier | 
**tax_number** | **String** | Account owner tax number. Can be CPF or CNPJ (only numbers). | 
**name** | **String** | Account owner name. | 
**payment_institution** | [**models::PaymentInstitution**](PaymentInstitution.md) | Recipient's bank account destination. | 
**is_default** | **bool** | Indicates if the recipient is the default one | 
**account** | [**models::PaymentRecipientAccount**](PaymentRecipientAccount.md) |  | 
**pix_key** | Option<**String**> | Pix key associated with the payment recipient | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


