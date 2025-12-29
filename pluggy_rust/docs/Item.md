# Item

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Primary identifier | 
**connector** | Option<[**models::Connector**](Connector.md)> |  | [optional]
**status** | **String** | Status of the Item | 
**execution_status** | **String** | Status of the sync execution | 
**error** | Option<[**models::ItemError**](Item_error.md)> |  | [optional]
**parameter** | Option<[**models::ConnectorCredential**](ConnectorCredential.md)> |  | [optional]
**user_action** | Option<[**models::ConnectorUserAction**](ConnectorUserAction.md)> |  | [optional]
**webhook_url** | Option<**String**> | Url to be notified of item changes | [optional]
**created_at** | Option<**String**> | Date of creation | [optional]
**updated_at** | Option<**String**> | Date of last modification | [optional]
**last_updated_at** | Option<**String**> | Date of last syncronization | [optional]
**status_detail** | Option<[**models::StatusDetail**](StatusDetail.md)> |  | [optional]
**next_auto_sync_at** | Option<**String**> | Date of next auto-sync, or null if auto-sync is disabled for this Item | [optional]
**consecutive_failed_login_attempts** | Option<**f64**> | Consecutives execution that ends up with a LOGIN_ERROR status | [optional]
**consent_expires_at** | Option<**String**> | Consent expiration date | [optional]
**products** | Option<**Vec<String>**> | Products collected by the item | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


