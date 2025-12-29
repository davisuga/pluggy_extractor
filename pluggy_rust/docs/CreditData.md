# CreditData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**level** | Option<**String**> | Card level (Black, Signature) | [optional]
**brand** | Option<**String**> | Card Brand (Visa, Mastercard, Elo) | [optional]
**balance_close_date** | Option<**String**> | Date when the balance was closed | [optional]
**balance_due_date** | Option<**String**> | Date when the balance is dued | [optional]
**available_credit_limit** | Option<**f64**> | Credit limit available to spent | [optional]
**balance_foreign_currency** | Option<**f64**> | Balance in USD | [optional]
**minimum_payment** | Option<**f64**> | Minimum payment due | [optional]
**credit_limit** | Option<**f64**> | Maximum amount that can be spent | [optional]
**status** | Option<**String**> | Credit card status | [optional]
**holder_type** | Option<**String**> | Credit card holder type | [optional]
**disaggregated_credit_limits** | Option<[**Vec<models::DisaggregatedCreditLimit>**](DisaggregatedCreditLimit.md)> | Disaggregated credit card limits | [optional]
**additional_cards** | Option<[**Vec<models::AdditionalCard>**](AdditionalCard.md)> | Additional credit cards associated with the main one | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


