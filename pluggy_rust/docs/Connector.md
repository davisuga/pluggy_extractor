# Connector

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **f64** | Primary identifier | 
**name** | Option<**String**> | Name of the institution | [optional]
**institution_url** | Option<**String**> | Homepage of the institution | [optional]
**image_url** | Option<**String**> | Image of the logo hosted by Pluggy | [optional]
**primary_color** | Option<**String**> | Primary color | [optional]
**r#type** | Option<**String**> | Type of institution | [optional]
**country** | Option<**String**> | Country located | [optional]
**credentials** | Option<[**Vec<models::ConnectorCredential>**](ConnectorCredential.md)> | Parameters required to start the connection | [optional]
**has_mfa** | Option<**bool**> | Does the connector require an MFA to execute? | [optional]
**products** | Option<**Vec<String>**> | Products supported by the connector | [optional]
**oauth** | Option<**bool**> | If 'true', the connector requires an Oauth flow to execute | [optional]
**oauth_url** | Option<**String**> | URL to perform Oauth flow if needed | [optional]
**reset_password_url** | Option<**String**> | URL to the financial institution to reset the password | [optional]
**health** | Option<[**models::ConnectorHealth**](ConnectorHealth.md)> |  | [optional]
**is_open_finance** | Option<**bool**> | Indicates if the connector uses the regulated Open Finance APIs | [optional]
**supports_payment_initiation** | Option<**bool**> | Indicates if the connector supports the payment initiation API | [optional]
**supports_scheduled_payments** | Option<**bool**> | Indicates if the connector supports scheduled payments | [optional]
**supports_smart_transfers** | Option<**bool**> | Indicates if the connector supports smart transfers | [optional]
**supports_boleto_management** | Option<**bool**> | Indicates if the connector supports boleto management | [optional]
**created_at** | Option<**String**> | Date of creation | [optional]
**updated_at** | Option<**String**> | Date of last modification | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


