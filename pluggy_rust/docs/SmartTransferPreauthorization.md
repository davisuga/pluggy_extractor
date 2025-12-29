# SmartTransferPreauthorization

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Preauthorization primary identifier | 
**status** | **String** | Preauthorization status | 
**consent_url** | Option<**String**> | Url to give the consent in the institution | [optional]
**client_preauthorization_id** | Option<**String**> | Client preauthorization identifier | [optional]
**callback_urls** | Option<[**models::SmartTransferCallbackUrls**](SmartTransferCallbackUrls.md)> |  | [optional]
**recipients** | [**Vec<models::PaymentRecipient>**](PaymentRecipient.md) |  | 
**connector** | [**models::Connector**](Connector.md) |  | 
**created_at** | **String** | Date when the preauthorization was created | 
**updated_at** | **String** | Date when the preauthorization was updated | 
**configuration** | Option<[**models::SmartTransferPreauthorizationConfiguration**](SmartTransferPreauthorizationConfiguration.md)> |  | [optional]
**error_detail** | Option<[**models::SmartTransferPreauthorizationErrorDetail**](SmartTransferPreauthorization_errorDetail.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


