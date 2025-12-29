# LoanInterestRate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**tax_type** | Option<**String**> | Tax type | [optional]
**interest_rate_type** | Option<**String**> | Interest rate type | [optional]
**tax_periodicity** | Option<**String**> | Tax periodicity | [optional]
**calculation** | Option<**String**> | Calculation basis | [optional]
**referential_rate_indexer_type** | Option<**String**> | Types of benchmark rates or indexers (https://openbanking-brasil.github.io/openapi/swagger-apis/loans/?urls.primaryName=2.0.1#model-EnumContractReferentialRateIndexerType) | [optional]
**referential_rate_indexer_sub_type** | Option<**String**> | Subtypes of benchmark rates or indexers (https://openbanking-brasil.github.io/openapi/swagger-apis/loans/?urls.primaryName=2.0.1#model-EnumContractReferentialRateIndexerSubType) | [optional]
**referential_rate_indexer_additional_info** | Option<**String**> | Free field to complement the information regarding the Type of reference rate or indexer | [optional]
**pre_fixed_rate** | Option<**f64**> | Pre-fixed rate applied under the credit modality contract. 1 = 100% | [optional]
**post_fixed_rate** | Option<**f64**> | Post-fixed rate applied under the credit modality contract. 1 = 100% | [optional]
**additional_info** | Option<**String**> | Text with additional information on the composition of agreed interest rates | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


