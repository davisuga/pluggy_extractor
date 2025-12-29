# Consent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Consent primary identifier | 
**item_id** | **String** | Primary identifier of the item associated to the consent | 
**products** | **Vec<String>** | Products to be collected in the connection | 
**open_finance_permissions_granted** | Option<**Vec<String>**> | Products consented by the user to be collected | [optional]
**created_at** | **String** | Date when the consent was given | 
**expires_at** | Option<**String**> | Date when the consent expires. Null if the consent doesn't expire | [optional]
**revoked_at** | Option<**String**> | Date when the consent was revoked | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


