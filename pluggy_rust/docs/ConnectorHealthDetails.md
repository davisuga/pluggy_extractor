# ConnectorHealthDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**connection_rate_last6_hours** | Option<**f64**> | A number from 0 to 100: the percentage of executions that succesfully connect to the institution: status of CONNECTION_ERROR,ERROR,SITE_NOT_AVAILABLE decrease the percentage. Any other status (like SUCCESS/LOGIN_ERROR) increase the percentage. The value will be null if there were no connections | [optional]
**connections_last6_hours** | Option<**f64**> | Amount of your connections for this connector during the last 6 hours. 0 if there were no connections | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


