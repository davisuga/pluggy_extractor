# CreateItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**connector_id** | **f64** | Primary identifier of the connector | 
**parameters** | [**models::CreateItemParameters**](CreateItem_parameters.md) |  | 
**webhook_url** | Option<**String**> | Url to be notified of item changes | [optional]
**client_user_id** | Option<**String**> | Client's external identifier for the user, it can be a ID, UUID or even an email. This is free for clients to use. | [optional]
**oauth_redirect_uri** | Option<**String**> | Redirect URI required for the Oauth flow | [optional]
**products** | Option<**Vec<String>**> | Products to be collected in the connection | [optional]
**avoid_duplicates** | Option<**bool**> | Avoids creating a new item if there is already one with the same credentials | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


