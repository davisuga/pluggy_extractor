# CreatePaymentCustomerRequestBody

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | Customer type | 
**name** | **String** | Customer name | 
**email** | Option<**String**> | Customer email | [optional]
**cpf** | Option<**String**> | Customer CPF | [optional]
**cnpj** | Option<**String**> | Customer CNPJ, if type is `BUSINESS` | [optional]
**connector_id** | Option<**f64**> | Default connector id to be used in the Pluggy's payment initiation flow (https://pay.pluggy.ai) by the payer. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


