# Weekly

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | Scheduled type | 
**start_date** | [**String**](string.md) | The start date of the validity of the scheduled payment authorization. | 
**day_of_week** | **String** | Day of the week on which each payment will occur. For instance, if set to 'MONDAY', the first payment will occur on the first monday after the startDate (or the same day, if it is already monday), and every monday after that. | 
**occurrences** | Option<**f64**> | Under the specified schedule frequency, how many payments will be scheduled to occur. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


