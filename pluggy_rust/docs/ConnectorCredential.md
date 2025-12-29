# ConnectorCredential

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | Name of the key | 
**label** | **String** | Label for input | 
**r#type** | **String** | Type of credential required | 
**assistive_text** | Option<**String**> | Text to help the user when completing the input | [optional]
**data** | Option<**String**> | Used to return base64 images | [optional]
**placeholder** | Option<**String**> | Placeholder text for the input | [optional]
**validation** | Option<**String**> | Regex validation for the user's input | [optional]
**validation_message** | Option<**String**> | Validation message when input doesn't match the regex | [optional]
**mfa** | Option<**bool**> | Credential is an MFA parameter and must be refreshed on each execution | [optional]
**options** | Option<[**Vec<models::CredentialSelectOption>**](CredentialSelectOption.md)> | List of possible values for the input | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


