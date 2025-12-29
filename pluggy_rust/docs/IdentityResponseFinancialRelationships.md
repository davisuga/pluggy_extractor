# IdentityResponseFinancialRelationships

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**start_date** | **String** | Date when the relationship with the institution started | 
**products_services_type** | **Vec<String>** | List of products and services that the client consumes | 
**procurators** | [**Vec<models::IdentityResponseFinancialRelationshipsProcuratorsInner>**](IdentityResponse_financialRelationships_procurators_inner.md) | List of procurators of the client | 
**accounts** | Option<[**Vec<models::IdentityResponseFinancialRelationshipsAccountsInner>**](IdentityResponse_financialRelationships_accounts_inner.md)> | List of accounts of the client with valid consent. Only accounts that have explicit user consent are returned. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


