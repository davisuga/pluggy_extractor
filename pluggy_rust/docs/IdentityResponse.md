# IdentityResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The ID of the identity to retrieve | 
**item_id** | [**uuid::Uuid**](uuid::Uuid.md) | Identifier of the item linked to the identity | 
**birth_date** | Option<**String**> | Date of birth | [optional]
**tax_number** | Option<**String**> | The tax ID (CNPJ) associated with the business account | [optional]
**document** | Option<**String**> | Primary document that identifies the owner | [optional]
**document_type** | Option<**String**> | Type of document collected | [optional]
**job_title** | Option<**String**> | Profession or Job information | [optional]
**full_name** | Option<**String**> | Name of the owner of the account | [optional]
**establishment_code** | Option<**String**> | Establishment code (only for PAYMENT_ACCOUNT connectors) | [optional]
**establishment_name** | Option<**String**> | Name of the establishment (only for PAYMENT_ACCOUNT connectors) | [optional]
**company_name** | Option<**String**> | For business connector, the name of the business | [optional]
**phone_numbers** | Option<[**Vec<models::PhoneNumber>**](PhoneNumber.md)> | List of phone numbers related to the account | [optional]
**emails** | Option<[**Vec<models::Email>**](Email.md)> | List of email addresses related to the account | [optional]
**addresses** | Option<[**Vec<models::Address>**](Address.md)> | List of addresses related to the account | [optional]
**relations** | Option<[**Vec<models::IdentityRelation>**](IdentityRelation.md)> | List of names related to the account | [optional]
**investor_profile** | Option<**String**> | Is a rating that indicates the investor personality and motivation for investing | [optional]
**qualifications** | Option<[**models::IdentityResponseQualifications**](IdentityResponse_qualifications.md)> |  | [optional]
**financial_relationships** | Option<[**models::IdentityResponseFinancialRelationships**](IdentityResponse_financialRelationships.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


