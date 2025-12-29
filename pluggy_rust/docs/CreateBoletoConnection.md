# CreateBoletoConnection

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**connector_id** | **i32** | Connector identifier. Check out the list of connectors, and if it has the flag 'supportsBoletoManagement' as true, it means it's possible to create a boleto connection with it. | 
**credentials** | **std::collections::HashMap<String, String>** | Credentials required for the connection. For Inter, they are clientId, clientSecret, certificate and privateKey, follow: https://docs.pluggy.ai/docs/connect-an-account#inter-pj | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


