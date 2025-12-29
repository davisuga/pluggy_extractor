# IssuedBoletoPayer

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**tax_number** | **String** | Payer tax number (CPF/CNPJ) | 
**person_type** | Option<**String**> | Type of person (individual or business) | [optional]
**name** | **String** | Payer name | 
**address_street** | Option<**String**> | Payer street address | [optional]
**address_number** | Option<**String**> | Payer address number | [optional]
**address_complement** | Option<**String**> | Additional address information | [optional]
**address_neighborhood** | Option<**String**> | Payer neighborhood | [optional]
**address_city** | Option<**String**> | Payer city | [optional]
**address_state** | **String** | Payer state | 
**address_zip_code** | **String** | Payer ZIP code | 
**email** | Option<**String**> | Payer email | [optional]
**ddd** | Option<**String**> | Payer area code | [optional]
**phone_number** | Option<**String**> | Payer phone number | [optional]
**amount_paid** | Option<**f64**> | Amount paid or null if it hasn't been paid yet | [optional]
**payment_origin** | Option<**String**> | Payment origin for the boleto | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


